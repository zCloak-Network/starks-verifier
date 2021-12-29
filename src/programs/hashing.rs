use super::{
    OpCode, ProgramBlock, BASE_CYCLE_LENGTH, HACC_NUM_ROUNDS, SPONGE_WIDTH as STATE_WIDTH,
};
use crate::utils::sponge;
use sp_std::vec::Vec;

// CONSTANTS
// ================================================================================================
pub const HACC_ROUND_OFFSET: usize = 1;
pub const NOOP_VALUE: u8 = OpCode::Noop as u8;

// PUBLIC FUNCTIONS
// ================================================================================================

/// Returns a hash of a sequence of program blocks.
pub fn hash_seq(blocks: &Vec<ProgramBlock>, suffix: &[u8], suffix_offset: usize) -> u128 {
    // initialize the state to all zeros
    let mut state = [0u128; STATE_WIDTH];

    // update the state with the hash of the first block, which must be a Span block
    state = match &blocks[0] {
        ProgramBlock::Span(block) => block.hash(state),
        _ => panic!("first block in a sequence must be a Span block"),
    };

    // update the state with hashes of all other blocks
    for block in blocks.iter().skip(1) {
        match block {
            ProgramBlock::Span(block) => {
                // for Span blocks, first do an extra round of acc_hash to ensure block
                // alignment on a 16 cycle boundary
                hash_op(&mut state, NOOP_VALUE, 0, BASE_CYCLE_LENGTH - 1);

                // then, update the state with the hash of the block
                state = block.hash(state);
            }
            _ => {
                // for control blocks, first get the hash of each block
                let (v0, v1) = match block {
                    ProgramBlock::Group(block) => block.get_hash(),
                    ProgramBlock::Switch(block) => block.get_hash(),
                    ProgramBlock::Loop(block) => block.get_hash(),
                    ProgramBlock::Span(_) => (0, 0), // can't happen
                };

                // then, merge the hash with the state using acc_hash procedure
                state = hash_acc(state[0], v0, v1);
            }
        };
    }

    // apply sequence suffix to ensure alignment on 16-cycle boundary;
    // for non-loop sequences, suffix will be just a single NOOP;
    // for loop bodies, suffix will be NOT ASSERT followed by 14 NOOPs;
    for i in 0..suffix.len() {
        hash_op(&mut state, suffix[i], 0, suffix_offset + i);
    }

    return state[0];
}

/// Merges an operation with the state of the sponge.
pub fn hash_op(state: &mut [u128; STATE_WIDTH], op_code: u8, op_value: u128, step: usize) {
    sponge::apply_round(state, op_code as u128, op_value, step);
}

/// Merges hash of a control block (v0, v1) into the hash of the parent block.
pub fn hash_acc(parent_hash: u128, v0: u128, v1: u128) -> [u128; STATE_WIDTH] {
    let mut state = [parent_hash, v0, v1, 0];
    for i in HACC_ROUND_OFFSET..(HACC_ROUND_OFFSET + HACC_NUM_ROUNDS) {
        hash_op(&mut state, NOOP_VALUE, 0, i);
    }
    return state;
}
