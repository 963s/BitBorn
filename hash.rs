use sha3::{Digest, Sha3_256};

pub fn sha3_256_hex(data: &[u8]) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let out = hasher.finalize();
    hex::encode(out)
}
