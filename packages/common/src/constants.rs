pub const DEFAULT_HIGHEST_BLOCK_REFRESH_MS: u64 = 1000;
pub const DEFAULT_NODE_PORT: u16 = 8080;
pub const DEFAULT_MAX_CONNECTIONS: usize = 1000;
pub const DEFAULT_MAX_BODY_SIZE: usize = 1024;
pub const DEFAULT_BLOCK_DURATION_SEC: u64 = 10;
pub const DEFAULT_UPDATE_DIFFICULTY_INTERVAL: u64 = 100;
pub const DEFAULT_DIFFICULTY_UPDATE_CAP: f64 = 4.0;

pub const ROUTE_GET_BLOCK: &str = "get::block";
pub const ROUTE_GET_HIGHEST_BLOCK: &str = "get::highest_block";
pub const ROUTE_GET_DIFFICULTY: &str = "get::difficulty";
pub const ROUTE_ADD_BLOCK: &str = "post::block";
