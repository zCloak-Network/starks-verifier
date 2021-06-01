mod trace;
mod constraints;
mod options;
mod verifier;
mod proof;
mod fri;
mod utils;

pub use trace::{ TraceState };

pub use constraints::{
    ConstraintEvaluator
};

pub use utils::{
    ConstraintCoefficients,
    CompositionCoefficients };

pub use options::ProofOptions;
pub use proof::{ StarkProof, DeepValues };
pub use verifier::{ verify };

const MAX_CONSTRAINT_DEGREE : usize = 8;