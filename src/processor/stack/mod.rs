use crate::{
    math::field,
    utils::hasher,
    OpHint,
    HASH_STATE_WIDTH, MIN_STACK_DEPTH, MAX_STACK_DEPTH,
};
use sp_std::vec::Vec;

#[cfg(test)]
mod tests;

// // TYPES AND INTERFACES
// // ================================================================================================
// pub struct Stack {
//     registers   : Vec<Vec<u128>>,
//     tape_a      : Vec<u128>,
//     tape_b      : Vec<u128>,
//     max_depth   : usize,
//     depth       : usize,
//     step        : usize,
// }

// STACK IMPLEMENTATION
// // ================================================================================================
// impl Stack {
//     /// Returns value of the current step pointer.
//     #[cfg(test)]
//     pub fn current_step(&self) -> usize {
//         return self.step;
//     }
// }
