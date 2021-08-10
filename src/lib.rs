#![no_std]
use log::debug;
use sp_std::{ops::Range, vec, vec::Vec};
use serde::{Serialize, Deserialize};

#[macro_use]
extern crate alloc;
use alloc::string::String;

// RE-EXPORTS
// ================================================================================================
pub mod crypto;
pub mod math;
pub mod utils;

mod stark;
pub use stark::{ StarkProof, ProofOptions, GenOutput, ProgramAssembly };
mod processor;
pub use processor::{ OpCode, OpHint };

mod programs;
pub use programs::{ Program, ProgramInputs, assembly, blocks };


extern crate console_error_panic_hook;

// extern crate wasm_bindgen;
// use wasm_bindgen::prelude::*;
use blocks::{ ProgramBlock, Span, Group, Switch, Loop };

extern crate web_sys;
// use wasm_bindgen_test::*;
use codec::{Decode, Encode};
use crate::alloc::string::ToString;

pub fn verify(program_hash: &[u8; 32], public_inputs: &[u128], outputs: &[u128], proof: &StarkProof) -> Result<bool, String>
{
    return stark::verify(program_hash, public_inputs, outputs, proof);
}



// pub fn main_test() {
//     log::trace!(
//         target: "starks-proofgen",
//         "This is a program to generate proof for 'Number over 20', please enter a number to be verified:
//         We choose 24 for you as a test. ",    
//     );

//     let number_to_be_verified: u128 = 24;

//     let inputs = ProgramInputs::new(&[], &[number_to_be_verified as u128], &[]);
//     let num_outputs = 1;
//     let expected_result = vec![if_over_twen(number_to_be_verified as u128)];
//     let options = ProofOptions::default();
//     let program = assembly::compile("
//     begin
//         push.20 read gt.128
//     end").unwrap();

//     log::trace!(
//         target: "starks-proofgen",
//         "This is a program to proof your number is over 20 or not ; expected result: {:?}" ,
//         expected_result , 
//     );


//     // execute the program and generate the proof of execution
//     let (outputs, proof) = starks_proofgen(&program, &inputs, num_outputs, &options);
//     // println!("--------------------------------");
//     log::trace!(
//         target: "starks-proofgen",
//         "Executed program with hash 0x{},Program output: {:?}" ,
//         hex::encode(program.hash()),outputs,
//     );

//     assert_eq!(expected_result, outputs, "Program result was computed incorrectly");
//     // serialize the proof to see how big it is
    
//     let proof_bytes = bincode::serialize(&proof).unwrap();
//     let _proof_hex = hex::encode(&proof_bytes);
//     // println!("proof_hex is {:?}",proof_hex);
    
//     log::trace!(
//         target: "starks-proofgen",
//         "Execution proof size: {} KB,Execution proof security: {} bits" ,
//         proof_bytes.len() / 1024, options.security_level(true),
//     );
// }

// fn if_over_twen(value: u128) -> u128{
//     log::trace!(
//         target: "starks-proofgen",
//         "your number is {:?}",
//         value,
//     );

//     if value > 20{
//         return 1;
//     }else{
//         return 0;
//     }
// }






// GLOBAL CONSTANTS
// ================================================================================================

pub const MAX_CONTEXT_DEPTH : usize = 16;
pub const MAX_LOOP_DEPTH    : usize = 8;
const MIN_TRACE_LENGTH      : usize = 16;
const MAX_REGISTER_COUNT    : usize = 128;
const MIN_EXTENSION_FACTOR  : usize = 16;
const BASE_CYCLE_LENGTH     : usize = 16;

const MIN_STACK_DEPTH       : usize = 8;
const MIN_CONTEXT_DEPTH     : usize = 1;
const MIN_LOOP_DEPTH        : usize = 1;

// PUSH OPERATION
// ------------------------------------------------------------------------------------------------
const PUSH_OP_ALIGNMENT     : usize = 8;

// HASH OPERATION
// ------------------------------------------------------------------------------------------------
const HASH_STATE_RATE       : usize = 4;
const HASH_STATE_CAPACITY   : usize = 2;
const HASH_STATE_WIDTH      : usize = HASH_STATE_RATE + HASH_STATE_CAPACITY;
const HASH_NUM_ROUNDS       : usize = 10;
const HASH_DIGEST_SIZE      : usize = 2;

// OPERATION SPONGE
// ------------------------------------------------------------------------------------------------
const SPONGE_WIDTH          : usize = 4;
const PROGRAM_DIGEST_SIZE   : usize = 2;
const HACC_NUM_ROUNDS       : usize = 14;

// DECODER LAYOUT
// ------------------------------------------------------------------------------------------------
//
//  ctr ╒═════ sponge ══════╕╒═══ cf_ops ══╕╒═══════ ld_ops ═══════╕╒═ hd_ops ╕╒═ ctx ══╕╒═ loop ═╕
//   0    1    2    3    4    5    6    7    8    9    10   11   12   13   14   15   ..   ..   ..
// ├────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┴────┤

const NUM_CF_OP_BITS        : usize = 3;
const NUM_LD_OP_BITS        : usize = 5;
const NUM_HD_OP_BITS        : usize = 2;

const NUM_CF_OPS            : usize = 8;
const NUM_LD_OPS            : usize = 32;
const NUM_HD_OPS            : usize = 4;

const OP_COUNTER_IDX        : usize = 0;
const SPONGE_RANGE          : Range<usize> = Range { start:  1, end:  5 };
const CF_OP_BITS_RANGE      : Range<usize> = Range { start:  5, end:  8 };
const LD_OP_BITS_RANGE      : Range<usize> = Range { start:  8, end: 13 };
const HD_OP_BITS_RANGE      : Range<usize> = Range { start: 13, end: 15 };

// STACK LAYOUT
// ------------------------------------------------------------------------------------------------
//
// ╒═══════════════════ user registers ════════════════════════╕
//    0      1    2    .................................    31
// ├─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┤

pub const MAX_PUBLIC_INPUTS : usize = 8;
pub const MAX_OUTPUTS       : usize = MAX_PUBLIC_INPUTS;
pub const MAX_STACK_DEPTH   : usize = 32;