mod constraints;
mod fri;
mod options;
mod proof;
mod prover;
mod trace;
mod utils;
mod verifier;

pub use trace::{TraceState, TraceTable};

pub use constraints::{ConstraintEvaluator, ConstraintPoly, ConstraintTable};

pub use utils::{CompositionCoefficients, ConstraintCoefficients};

pub use options::ProofOptions;
pub use proof::{DeepValues, GenOutput, ProgramAssembly, StarkProof};
pub use prover::prove;
pub use verifier::verify;

const MAX_CONSTRAINT_DEGREE: usize = 8;
