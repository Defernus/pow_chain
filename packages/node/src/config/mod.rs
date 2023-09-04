use dotenv::dotenv;
use pow_chain_core::{
    constants::{DEFAULT_MAX_BODY_SIZE, DEFAULT_MAX_CONNECTIONS, DEFAULT_NODE_PORT},
    env::{read_env, read_optional_env},
};

pub struct Config {
    pub secret: String,
    pub port: u16,
    pub max_connections: usize,
    pub max_body_size: usize,
}

impl Config {
    pub fn new() -> Self {
        dotenv().unwrap();

        Self {
            secret: read_env("NODE_SECRET"),
            port: read_optional_env("NODE_PORT").unwrap_or(DEFAULT_NODE_PORT),
            max_connections: read_optional_env("NODE_MAX_CONNECTIONS")
                .unwrap_or(DEFAULT_MAX_CONNECTIONS),
            max_body_size: read_optional_env("NODE_MAX_BODY_SIZE").unwrap_or(DEFAULT_MAX_BODY_SIZE),
        }
    }
}
