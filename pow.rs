use crate::crypto::hash::sha3_256_hex;
use crate::types::BlockHeader;

pub fn header_bytes(mut h: BlockHeader, nonce: u64) -> Vec<u8> {
    h.nonce = nonce;
    serde_json::to_vec(&h).expect("serialize header")
}

// Very toy PoW for demo only (NOT secure)
pub fn mine(header: BlockHeader, difficulty_leading_zeros: usize) -> (u64, String) {
    let target_prefix = "0".repeat(difficulty_leading_zeros);
    let mut nonce = 0u64;
    loop {
        let bytes = header_bytes(header.clone(), nonce);
        let hash_hex = sha3_256_hex(&bytes);
        if hash_hex.starts_with(&target_prefix) {
            return (nonce, hash_hex);
        }
        nonce = nonce.wrapping_add(1);
        if nonce == u64::MAX {
            panic!("Exhausted nonce space in demo miner");
        }
    }
}
