#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
