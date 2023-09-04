use serde::{Deserialize, Serialize};

use crate::node::block::Block;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct HighestBlockResponse {
    pub block: Block,
    pub height: usize,
    pub difficulty: f64,
}
