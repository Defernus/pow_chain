use std::time::SystemTime;

use pow_common::hash::{hash_block_data, validate_hash};
use pow_node::{config::Config, node::Node};

fn mine_block(cfg: &Config, node: &mut Node, data: String) {
    let start_time = SystemTime::now();

    let mut nonce = 0;

    let prev_hash = node.get_highest_block().hash();

    let current_difficulty = node.get_difficulty();

    loop {
        let hash = hash_block_data(&data, &prev_hash, nonce);

        if validate_hash(&hash, current_difficulty) {
            node.add_block(cfg, data, nonce).unwrap();
            break;
        }

        nonce += 1;
    }

    let end_time = SystemTime::now();

    log::info!(
        "block mined:\n\ttime {:?}\n\tnonce {}\n\tdifficulty: {}",
        end_time.duration_since(start_time).unwrap(),
        nonce,
        current_difficulty,
    );
}

fn main() {
    let cfg = Config::new();
    let mut node = Node::new("test node");

    log::info!("start mining");

    let mut index = 0;

    loop {
        mine_block(&cfg, &mut node, format!("test block {index}"));

        index += 1;
    }
}
