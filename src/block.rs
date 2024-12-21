use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub prev_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, data: String, prev_hash: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let hash = Self::calculate_hash(index, timestamp, &data, &prev_hash);

        Self {
            index,
            timestamp,
            data,
            prev_hash,
            hash,
        }
    }

    pub fn calculate_hash(index: u64, timestamp: u64, data: &str, prev_hash: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, data, prev_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }
}
