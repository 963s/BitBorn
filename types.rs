use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    pub parent_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub nonce: u64,
}

impl BlockHeader {
    pub fn example() -> Self {
        Self {
            parent_hash: "00".repeat(16),
            merkle_root: "11".repeat(16),
            timestamp: 0,
            nonce: 0,
        }
    }
}
