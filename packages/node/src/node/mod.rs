use std::time::{Duration, SystemTime};

use crate::config::Config;

use self::{
    block::Block,
    error::{NodeError, NodeResult},
};

pub mod block;
pub mod error;

pub struct Node {
    blocks: Vec<Block>,
    /// Time of the first block.
    root_time: SystemTime,
    /// average amount of hashes per block
    difficulty: f64,
}

impl Node {
    pub fn new(data: String) -> NodeResult<Self> {
        let first_block = Block::first_block(data);

        Ok(Self {
            root_time: SystemTime::now(),
            blocks: vec![first_block],
            difficulty: 2.0,
        })
    }

    pub fn add_block(
        &mut self,
        cfg: &Config,
        data: String,
        nonce: u64,
        prev_block_height: usize,
    ) -> NodeResult<()> {
        let prev_block = self.get_block(prev_block_height)?;

        let block = prev_block.validate_next_block(self.difficulty, self.root_time, data, nonce)?;

        if block.height % cfg.update_difficulty_interval == 0 {
            self.update_difficulty(cfg);
        }

        self.blocks.push(block);

        Ok(())
    }

    pub fn get_block(&self, height: usize) -> NodeResult<&Block> {
        let block = self.blocks.get(height).ok_or(NodeError::BlockNotFound)?;

        Ok(block)
    }

    pub fn update_difficulty(&mut self, cfg: &Config) {
        let blocks_to_check = if self.blocks.len() < cfg.update_difficulty_interval as usize {
            self.blocks.len()
        } else {
            cfg.update_difficulty_interval as usize
        };

        let total_duration = self.blocks[self.blocks.len() - 1].time_from_root
            - self.blocks[self.blocks.len() - blocks_to_check].time_from_root;

        let expected_duration = cfg.target_block_duration * cfg.update_difficulty_interval as u32;

        let new_difficulty =
            self.difficulty / total_duration.as_secs_f64() * expected_duration.as_secs_f64();

        let max_difficulty = self.difficulty * cfg.difficulty_update_cap;
        let min_difficulty = self.difficulty / cfg.difficulty_update_cap;

        self.difficulty = new_difficulty.clamp(max_difficulty, min_difficulty);
    }
}
