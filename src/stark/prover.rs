use super::{
    constraints::{ConstraintPoly, ConstraintTable},
    fri,
    trace::{TraceState, TraceTable},
    utils, CompositionCoefficients, DeepValues, ProofOptions, StarkProof, MAX_CONSTRAINT_DEGREE,
};
use crate::{
    crypto::MerkleTree,
    math::{fft, field, polynom},
};
use log::debug;

use sp_std::vec::Vec;
// use wasm_bindgen_test::*;

// PROVER FUNCTION
// ================================================================================================

pub fn prove(
    trace: &mut TraceTable,
    inputs: &[u128],
    outputs: &[u128],
    options: &ProofOptions,
) -> StarkProof {
    // 1 ----- extend execution trace -------------------------------------------------------------

    // build LDE domain and LDE twiddles (for FFT evaluation over LDE domain)
    let lde_root = field::get_root_of_unity(trace.domain_size());

    let lde_domain = field::get_power_series(lde_root, trace.domain_size());

    let lde_twiddles = twiddles_from_domain(&lde_domain);

    // extend the execution trace registers to LDE domain
    trace.extend(&lde_twiddles);
    debug!(
        "Extended execution trace from {} to {} steps",
        trace.unextended_length(),
        trace.domain_size()
    );

    // 2 ----- build Merkle tree from the extended execution trace ------------------------------------
    let trace_tree = trace.build_merkle_tree(options.hash_fn());

    // 3 ----- evaluate constraints ---------------------------------------------------------------

    // initialize constraint evaluation table
    let mut constraints = ConstraintTable::new(&trace, trace_tree.root(), inputs, outputs);

    // allocate space to hold current and next states for constraint evaluations
    let mut current = TraceState::new(trace.ctx_depth(), trace.loop_depth(), trace.stack_depth());
    let mut next = TraceState::new(trace.ctx_depth(), trace.loop_depth(), trace.stack_depth());

    // we don't need to evaluate constraints over the entire extended execution trace; we need
    // to evaluate them over the domain extended to match max constraint degree - thus, we can
    // skip most trace states for the purposes of constraint evaluation.
    let stride = trace.extension_factor() / MAX_CONSTRAINT_DEGREE;
    for i in (0..trace.domain_size()).step_by(stride) {
        // TODO: this loop should be parallelized and also potentially optimized to avoid copying
        // next state from the trace table twice

        // copy current and next states from the trace table; next state may wrap around the
        // execution trace (close to the end of the trace)
        trace.fill_state(&mut current, i);
        trace.fill_state(
            &mut next,
            (i + trace.extension_factor()) % trace.domain_size(),
        );

        // evaluate the constraints
        constraints.evaluate(&current, &next, lde_domain[i], i / stride);
    }

    debug!(
        "Evaluated {} constraints over domain of {} elements",
        constraints.constraint_count(),
        constraints.evaluation_domain_size()
    );

    // 4 ----- convert constraint evaluations into a polynomial -----------------------------------
    let constraint_poly = constraints.combine_polys();
    debug!(
        "Converted constraint evaluations into a single polynomial of degree {}",
        constraint_poly.degree()
    );

    // 5 ----- build Merkle tree from constraint polynomial evaluations ---------------------------

    // evaluate constraint polynomial over the evaluation domain
    let constraint_evaluations = constraint_poly.eval(&lde_twiddles);

    // put evaluations into a Merkle tree; 4 evaluations per leaf
    let constraint_evaluations = evaluations_to_leaves(constraint_evaluations);
    let constraint_tree = MerkleTree::new(constraint_evaluations, options.hash_fn());

    // 6 ----- build and evaluate deep composition polynomial -------------------------------------

    // combine trace and constraint polynomials into the final deep composition polynomial
    let seed = constraint_tree.root();
    let (composition_poly, deep_values) = build_composition_poly(&trace, constraint_poly, seed);

    // evaluate the composition polynomial over LDE domain
    let mut composed_evaluations = composition_poly;
    debug_assert!(
        composed_evaluations.capacity() == lde_domain.len(),
        "invalid composition polynomial capacity"
    );
    unsafe {
        composed_evaluations.set_len(composed_evaluations.capacity());
    }
    polynom::eval_fft_twiddles(&mut composed_evaluations, &lde_twiddles, true);

    debug!(
        "Built composition polynomial and evaluated it over domain of {} elements",
        composed_evaluations.len()
    );

    // 7 ----- compute FRI layers for the composition polynomial ----------------------------------
    let composition_degree = utils::get_composition_degree(trace.unextended_length());
    debug_assert!(composition_degree == polynom::infer_degree(&composed_evaluations));
    let (fri_trees, fri_values) = fri::reduce(&composed_evaluations, &lde_domain, options);

    // 8 ----- determine query positions -----------------------------------------------------------

    // combine all FRI layer roots into a single vector
    let mut fri_roots: Vec<u8> = Vec::new();
    for tree in fri_trees.iter() {
        tree.root().iter().for_each(|&v| fri_roots.push(v));
    }

    // derive a seed from the combined roots
    let mut seed = [0u8; 32];
    options.hash_fn()(&fri_roots, &mut seed);

    // apply proof-of-work to get a new seed
    let (seed, pow_nonce) = utils::find_pow_nonce(seed, &options);

    // generate pseudo-random query positions

    let positions = utils::compute_query_positions(&seed, lde_domain.len(), options);

    debug!(
        "Determined {} query positions from seed {}",
        positions.len(),
        hex::encode(seed)
    );

    // 9 ----- build proof object -----------------------------------------------------------------

    // generate FRI proof
    let fri_proof = fri::build_proof(fri_trees, fri_values, &positions);

    // built a list of trace evaluations at queried positions
    let trace_evaluations = trace.get_register_values_at(&positions);

    // build a list of constraint positions
    let constraint_positions = utils::map_trace_to_constraint_positions(&positions);

    // build the proof object
    let proof = StarkProof::new(
        trace_tree.root(),
        trace_tree.prove_batch(&positions),
        trace_evaluations,
        constraint_tree.root(),
        constraint_tree.prove_batch(&constraint_positions),
        deep_values,
        fri_proof,
        pow_nonce,
        trace.get_last_state().op_counter(),
        trace.ctx_depth(),
        trace.loop_depth(),
        trace.stack_depth(),
        &options,
    );

    return proof;
}

// HELPER FUNCTIONS
// ================================================================================================
fn twiddles_from_domain(domain: &[u128]) -> Vec<u128> {
    let mut twiddles = domain[..(domain.len() / 2)].to_vec();
    fft::permute(&mut twiddles);
    return twiddles;
}

/// Re-interpret vector of 16-byte values as a vector of 32-byte arrays
fn evaluations_to_leaves(evaluations: Vec<u128>) -> Vec<[u8; 32]> {
    assert!(
        evaluations.len() % 2 == 0,
        "number of values must be divisible by 2"
    );
    let mut v = sp_std::mem::ManuallyDrop::new(evaluations);
    let p = v.as_mut_ptr();
    let len = v.len() / 2;
    let cap = v.capacity() / 2;
    return unsafe { Vec::from_raw_parts(p as *mut [u8; 32], len, cap) };
}

fn build_composition_poly(
    trace: &TraceTable,
    constraint_poly: ConstraintPoly,
    seed: &[u8; 32],
) -> (Vec<u128>, DeepValues) {
    // pseudo-randomly selection deep point z and coefficients for the composition
    let z = field::prng(*seed);
    let coefficients = CompositionCoefficients::new(*seed);

    // divide out deep point from trace polynomials and merge them into a single polynomial
    let (mut result, s1, s2) = trace.get_composition_poly(z, &coefficients);

    // divide out deep point from constraint polynomial and merge it into the result
    constraint_poly.merge_into(&mut result, z, &coefficients);

    return (
        result,
        DeepValues {
            trace_at_z1: s1,
            trace_at_z2: s2,
        },
    );
}
