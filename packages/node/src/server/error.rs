use serde::{Deserialize, Serialize};

use crate::node::error::NodeError;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum ServerError {
    NodeError(NodeError),
    UnknownCommand,
    MaxConnectionsReached,
    MaxBodySizeReached,
    InvalidPayload { argument: usize, message: String },
}

impl From<NodeError> for ServerError {
    fn from(err: NodeError) -> Self {
        Self::NodeError(err)
    }
}

pub type ServerResult<T> = Result<T, ServerError>;
