use crate::{
    NUM_CF_OP_BITS, NUM_LD_OP_BITS, NUM_HD_OP_BITS,
    SPONGE_WIDTH
};
use sp_std::vec::Vec;

#[cfg(test)]
mod tests;

// TYPES AND INTERFACES
// ================================================================================================
// pub struct Decoder {

//     step        : usize,

//     op_counter  : Vec<u128>,
//     sponge_trace: [Vec<u128>; SPONGE_WIDTH],
//     sponge      : [u128; SPONGE_WIDTH],

//     cf_op_bits  : [Vec<u128>; NUM_CF_OP_BITS],
//     ld_op_bits  : [Vec<u128>; NUM_LD_OP_BITS],
//     hd_op_bits  : [Vec<u128>; NUM_HD_OP_BITS],

//     ctx_stack   : Vec<Vec<u128>>,
//     ctx_depth   : usize,

//     loop_stack  : Vec<Vec<u128>>,
//     loop_depth  : usize,
// }

// // DECODER IMPLEMENTATION
// // ================================================================================================
// impl Decoder {
//     /// Returns value of the current step pointer.
//     #[cfg(test)]
//     pub fn current_step(&self) -> usize {
//         return self.step;
//     }

//     /// Returns the state of the stack at the specified `step`.
//     #[cfg(test)]
//     pub fn get_state(&self, step: usize) -> Vec<u128> {
//         let mut state = Vec::new();

//         state.push(self.op_counter[step]);
//         for register in self.sponge_trace.iter() { state.push(register[step]); }
//         for register in self.cf_op_bits.iter()   { state.push(register[step]); }
//         for register in self.ld_op_bits.iter()   { state.push(register[step]); }
//         for register in self.hd_op_bits.iter()   { state.push(register[step]); }
//         for register in self.ctx_stack.iter()    { state.push(register[step]); }
//         for register in self.loop_stack.iter()   { state.push(register[step]); }

//         return state;
//     }
// }