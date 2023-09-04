use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum NodeError {
    /// Block with given height not found
    BlockNotFound,
    /// First block already exists
    ///
    /// This error is returned when trying to init blockchain with a block
    /// that has `prev_block_id`
    FirstBlockAlreadyExists,
    /// Invalid difficulty
    ///
    /// This error is returned when trying to add a block that has `nonce`
    InvalidDifficulty,
    /// Invalid time from previous root block
    ///
    /// This error is returned when trying to add a block that has `time_from_root`
    /// less than previous block or if `time_from_root` not zero for the first block
    InvalidTimeFromRoot,
}

pub type NodeResult<T> = Result<T, NodeError>;

impl From<NodeError> for String {
    fn from(err: NodeError) -> Self {
        match err {
            NodeError::BlockNotFound => "block not found".to_string(),
            NodeError::FirstBlockAlreadyExists => "first block already exists".to_string(),
            NodeError::InvalidDifficulty => "invalid difficulty".to_string(),
            NodeError::InvalidTimeFromRoot => "invalid time from root".to_string(),
        }
    }
}
