use sp_std::slice;
use crate::stark::{ ProofOptions };
use alloc::string::String;


pub fn verify_pow_nonce(seed: [u8; 32], nonce: u64, options: &ProofOptions) -> Result<[u8; 32], String> {

    let hash = options.hash_fn();

    // append nonce to seed for hashing
    let mut input_bytes = [0; 64];
    input_bytes[0..32].copy_from_slice(&seed);
    input_bytes[32..40].copy_from_slice(&nonce.to_le_bytes());

    let output = [0u64; 4];
    let mut output_bytes: &mut [u8] = unsafe {
        slice::from_raw_parts_mut(output.as_ptr() as *mut u8, output.len() * 8)
    };

    hash(&input_bytes, &mut output_bytes);
    if output[0].trailing_zeros() < options.grinding_factor() {
        return Err(String::from("seed proof-of-work verification failed"));
    }

    let mut result = [0; 32];
    result.copy_from_slice(output_bytes);

    return Ok(result);
}