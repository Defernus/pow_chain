use std::time::Duration;

use dotenvy::dotenv;
use pow_common::{
    constants::{
        DEFAULT_BLOCK_DURATION_SEC, DEFAULT_DIFFICULTY_UPDATE_CAP, DEFAULT_MAX_BODY_SIZE,
        DEFAULT_MAX_CONNECTIONS, DEFAULT_NODE_PORT, DEFAULT_UPDATE_DIFFICULTY_INTERVAL,
    },
    env::{read_env, read_optional_env},
};

pub struct Config {
    pub port: u16,
    pub max_connections: usize,
    pub max_body_size: usize,
    pub target_block_duration: Duration,
    pub update_difficulty_interval: u64,
    pub difficulty_update_cap: f64,
    pub words_of_wisdom: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        dotenv().unwrap();

        Self {
            port: read_optional_env("NODE_PORT").unwrap_or(DEFAULT_NODE_PORT),
            max_connections: read_optional_env("NODE_MAX_CONNECTIONS")
                .unwrap_or(DEFAULT_MAX_CONNECTIONS),
            max_body_size: read_optional_env("NODE_MAX_BODY_SIZE").unwrap_or(DEFAULT_MAX_BODY_SIZE),
            target_block_duration: Duration::from_secs(
                read_optional_env("TARGET_BLOCK_DURATION_SEC")
                    .unwrap_or(DEFAULT_BLOCK_DURATION_SEC),
            ),
            update_difficulty_interval: read_optional_env("UPDATE_DIFFICULTY_INTERVAL")
                .unwrap_or(DEFAULT_UPDATE_DIFFICULTY_INTERVAL),
            difficulty_update_cap: read_optional_env("UPDATE_DIFFICULTY_CAP")
                .unwrap_or(DEFAULT_DIFFICULTY_UPDATE_CAP),
            words_of_wisdom: read_env::<String, _>("WORDS_OF_WISDOM")
                .split('\n')
                .map(String::from)
                .filter(|s| !s.is_empty())
                .collect(),
        }
    }
}

impl Config {
    /// returns a random word of wisdom
    pub fn get_word_of_wisdom(&self) -> String {
        let index = rand::random::<usize>() % self.words_of_wisdom.len();
        self.words_of_wisdom[index].clone()
    }
}
