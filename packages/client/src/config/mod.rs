use std::time::Duration;

use dotenvy::dotenv;
use pow_common::{
    constants::{DEFAULT_HIGHEST_BLOCK_REFRESH_MS, DEFAULT_NODE_PORT},
    env::{read_env, read_optional_env},
};

pub struct Config {
    pub name: String,
    pub node_host: String,
    pub node_port: u16,
    pub highest_block_refresh: Duration,
}

impl Default for Config {
    fn default() -> Self {
        dotenv().unwrap();

        Self {
            name: read_env("CLIENT_NAME"),
            node_host: read_env("NODE_HOST"),
            node_port: read_optional_env("NODE_PORT").unwrap_or(DEFAULT_NODE_PORT),
            highest_block_refresh: Duration::from_millis(
                read_optional_env("HIGHEST_BLOCK_REFRESH_SECONDS")
                    .unwrap_or(DEFAULT_HIGHEST_BLOCK_REFRESH_MS),
            ),
        }
    }
}
