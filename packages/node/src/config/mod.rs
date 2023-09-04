use dotenv::dotenv;
use pow_chain_core::env::{read_env, read_optional_env};

pub struct Config {
    pub secret: String,
    pub port: u16,
}

impl Config {
    pub fn new() -> Self {
        dotenv().unwrap();

        Self {
            secret: read_env("NODE_SECRET"),
            port: read_optional_env("NODE_PORT").unwrap_or(8080),
        }
    }
}
