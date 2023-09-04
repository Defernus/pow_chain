use std::time::{Duration, SystemTime};

use pow_core::hash::{hash_block_data, validate_hash};

use super::error::{NodeError, NodeResult};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    /// Text data of the block
    pub data: String,
    /// Block height. Height of the first block is 0, height of the next block
    /// is 1 more than previous block
    pub height: u64,
    /// Time from the root block
    pub time_from_root: Duration,
    /// Hash of the previous block
    pub prev_hash: String,
    /// An artificially generated number used as a counter during the mining process
    pub nonce: u64,
}

impl Block {
    pub fn first_block(data: String) -> Self {
        Self {
            data,
            height: 0,
            time_from_root: Duration::from_secs(0),
            prev_hash: String::new(),
            nonce: 0,
        }
    }

    pub fn validate_next_block(
        &self,
        difficulty: f64,
        root_time: SystemTime,
        data: String,
        nonce: u64,
    ) -> NodeResult<Self> {
        let time_from_root = SystemTime::now().duration_since(root_time).unwrap();
        if time_from_root < self.time_from_root {
            panic!("Time went backwards");
        }

        let prev_hash = self.hash();
        let hash = hash_block_data(&data, &prev_hash, nonce);

        if !validate_hash(&hash, difficulty) {
            return Err(NodeError::InvalidDifficulty);
        }

        Ok(Block {
            data,
            height: self.height + 1,
            nonce,
            prev_hash,
            time_from_root,
        })
    }

    pub fn hash(&self) -> String {
        hash_block_data(&self.data, &self.prev_hash, self.nonce)
    }
}
