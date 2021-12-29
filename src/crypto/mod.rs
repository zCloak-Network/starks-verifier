pub mod hash;

mod merkle;
pub use merkle::{build_merkle_nodes, BatchMerkleProof, MerkleTree};

pub type HashFunction = fn(&[u8], &mut [u8]);
