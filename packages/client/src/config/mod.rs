use dotenv::dotenv;
use pow_core::{
    constants::DEFAULT_NODE_PORT,
    env::{read_env, read_optional_env},
};

pub struct Config {
    pub node_host: String,
    pub node_port: u16,
}

impl Config {
    pub fn new() -> Self {
        dotenv().unwrap();

        Self {
            node_host: read_env("NODE_HOST"),
            node_port: read_optional_env("NODE_PORT").unwrap_or(DEFAULT_NODE_PORT),
        }
    }
}
