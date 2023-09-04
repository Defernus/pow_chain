use std::io::prelude::*;
use std::time::{Duration, SystemTime};
use std::{io::Write, net::TcpStream};

use pow_client::config::Config;
use pow_common::constants::{ROUTE_ADD_BLOCK, ROUTE_GET_HIGHEST_BLOCK};
use pow_common::hash::{hash_block_data, validate_hash};
use pow_common::update_log_level::update_log_level;
use pow_node::node::error::NodeError;
use pow_node::server::error::{ServerError, ServerResult};
use pow_node::server::responses::HighestBlockResponse;

const BUFFER_SIZE: usize = 1024;

fn send_data(socket: &mut TcpStream, data: &str) {
    socket.write_all(data.to_string().as_bytes()).unwrap();
    socket.flush().unwrap();
}

fn read_raw(socket: &mut TcpStream) -> String {
    let mut buffer = [0; BUFFER_SIZE];
    let mut result: Vec<u8> = vec![];

    while let Ok(size) = socket.read(&mut buffer) {
        if size == 0 {
            break;
        }

        result.extend_from_slice(&buffer[..size]);

        if size < BUFFER_SIZE {
            break;
        }
    }

    String::from_utf8(result).unwrap()
}

fn read_highest_block(socket: &mut TcpStream) -> ServerResult<HighestBlockResponse> {
    let resp = read_raw(socket);

    serde_json::from_str(&resp).unwrap()
}

fn get_highest_block(socket: &mut TcpStream) -> ServerResult<HighestBlockResponse> {
    send_data(socket, ROUTE_GET_HIGHEST_BLOCK);

    read_highest_block(socket)
}

fn mine_block(
    socket: &mut TcpStream,
    last_block: Option<HighestBlockResponse>,
    data: &str,
    block_refresh: Duration,
) -> u64 {
    let mut nonce = 0;

    let mut highest_block = last_block.unwrap_or_else(|| get_highest_block(socket).unwrap());

    let start_time = SystemTime::now();
    let mut last_update = start_time;

    let prev_hash = highest_block.block.hash();

    loop {
        // refresh highest block
        if SystemTime::now().duration_since(last_update).unwrap() > block_refresh {
            let new_block = get_highest_block(socket).unwrap();

            if new_block.height != highest_block.height {
                highest_block = new_block;
                nonce = 0;
            }
            last_update = SystemTime::now();
        }

        let hash = hash_block_data(data, &prev_hash, nonce);

        if validate_hash(&hash, highest_block.difficulty) {
            let end_time = SystemTime::now();

            log::debug!(
                "block found:\n\ttime {:?}\n\tnonce {}\n\tdifficulty: {}",
                end_time.duration_since(start_time).unwrap(),
                nonce,
                highest_block.difficulty,
            );

            return nonce;
        }

        nonce += 1;
    }
}

pub fn main() {
    let cfg = Config::default();

    update_log_level();

    let addr = format!("{}:{}", cfg.node_host, cfg.node_port);

    log::info!("connecting to: {addr}");

    let connection = TcpStream::connect(addr).unwrap();

    let mut block_count = 0;
    let mut last_block: Option<HighestBlockResponse> = None;

    loop {
        let mut socket = connection.try_clone().unwrap();

        let data = format!("{} block by {}", block_count, cfg.name);

        let nonce = mine_block(
            &mut socket,
            last_block.clone(),
            &data,
            cfg.highest_block_refresh,
        );

        let msg = format!("{ROUTE_ADD_BLOCK}\n{data}\n{nonce}");
        send_data(&mut socket, &msg);
        match read_highest_block(&mut socket) {
            Ok(block) => {
                log::debug!("block mined:\n\t{block:?}");

                log::info!("FOUND WORD OF WISDOM: {}", block.block.wow);
                last_block = Some(block);
            }
            Err(ServerError::NodeError(NodeError::InvalidDifficulty)) => {
                log::warn!("block already mined");
            }
            Err(err) => {
                log::error!("failed to mine block: {err:?}");
            }
        }

        block_count += 1;
    }
}
