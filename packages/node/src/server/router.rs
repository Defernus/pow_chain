use serde::{Deserialize, Serialize};
use std::sync::Arc;

use pow_core::constants::{
    ROUTE_ADD_BLOCK, ROUTE_GET_BLOCK, ROUTE_GET_DIFFICULTY, ROUTE_GET_HIGHEST_BLOCK,
};
use tokio::{
    net::TcpStream,
    sync::{mpsc, Mutex},
};

use crate::{
    config::Config,
    node::{block::Block, Node},
    server::{error::ServerError, helpers::send_response},
};

use super::{error::ServerResult, helpers::send_err};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct HighestBlockResponse {
    pub block: Block,
    pub height: usize,
}

pub struct Router {
    cfg: Arc<Config>,
    socket: Arc<Mutex<TcpStream>>,
    node: Arc<Mutex<Node>>,
    rx: mpsc::Receiver<Option<String>>,
}

impl Router {
    pub fn new(
        cfg: Arc<Config>,
        socket: Arc<Mutex<TcpStream>>,
        rx: mpsc::Receiver<Option<String>>,
        node: Arc<Mutex<Node>>,
    ) -> Self {
        Self {
            socket,
            rx,
            node,
            cfg,
        }
    }

    pub async fn run(&mut self) {
        while self.handle_command().await {}

        self.rx.close();
    }

    async fn next_msg(&mut self) -> Option<String> {
        self.rx.recv().await.unwrap()
    }

    async fn handle_command(&mut self) -> bool {
        let command = if let Some(msg) = self.next_msg().await {
            log::debug!("received message: {}", msg);
            msg
        } else {
            return false;
        };

        let response = match command.as_str() {
            ROUTE_GET_BLOCK => self.route_get_block().await,
            ROUTE_GET_DIFFICULTY => self.route_get_difficulty().await,
            ROUTE_GET_HIGHEST_BLOCK => self.route_get_highest_block().await,
            ROUTE_ADD_BLOCK => self.route_add_block().await,
            _ => {
                log::debug!("unknown command: {}", command);

                send_err(self.socket.clone(), ServerError::UnknownCommand)
                    .await
                    .unwrap();

                return false;
            }
        };

        match response {
            Ok(response) => {
                send_response(self.socket.clone(), response).await.unwrap();
                true
            }
            Err(err) => {
                send_err(self.socket.clone(), err).await.unwrap();
                false
            }
        }
    }

    async fn route_get_block(&mut self) -> ServerResult<String> {
        let block_height = self.next_msg().await.unwrap();
        let block_height =
            block_height
                .parse::<usize>()
                .map_err(|e| ServerError::InvalidPayload {
                    message: e.to_string(),
                    argument: 0,
                })?;

        let node: tokio::sync::MutexGuard<'_, Node> = self.node.lock().await;

        let block = node.get_block(block_height)?;

        Ok(serde_json::to_string(block).unwrap())
    }

    async fn route_get_difficulty(&mut self) -> ServerResult<String> {
        let node: tokio::sync::MutexGuard<'_, Node> = self.node.lock().await;

        let difficulty = node.get_difficulty();

        Ok(difficulty.to_string())
    }

    async fn route_get_highest_block(&mut self) -> ServerResult<String> {
        let node: tokio::sync::MutexGuard<'_, Node> = self.node.lock().await;

        let highest_block = node.get_highest_block();
        let height = node.get_height();

        Ok(serde_json::to_string(&HighestBlockResponse {
            block: highest_block.clone(),
            height,
        })
        .unwrap())
    }

    async fn route_add_block(&mut self) -> ServerResult<String> {
        let data = self.next_msg().await.unwrap();
        let nonce = self.next_msg().await.unwrap();
        let nonce = nonce
            .parse::<u64>()
            .map_err(|e| ServerError::InvalidPayload {
                message: e.to_string(),
                argument: 1,
            })?;

        let mut node: tokio::sync::MutexGuard<'_, Node> = self.node.lock().await;

        let block = node.add_block(&self.cfg, data, nonce)?.clone();
        let height = node.get_height();

        Ok(serde_json::to_string(&HighestBlockResponse { block, height }).unwrap())
    }
}
