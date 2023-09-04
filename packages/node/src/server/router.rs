use std::sync::Arc;

use pow_common::constants::{
    ROUTE_ADD_BLOCK, ROUTE_GET_BLOCK, ROUTE_GET_DIFFICULTY, ROUTE_GET_HIGHEST_BLOCK,
};
use tokio::{
    net::TcpStream,
    sync::{mpsc, Mutex},
};

use crate::{
    config::Config,
    node::{block::Block, Node},
    server::{
        error::{ServerError, ServerResult},
        helpers::{send_err, send_response},
        responses::HighestBlockResponse,
    },
};

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

        // wait 1 second
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        match command.as_str() {
            ROUTE_GET_BLOCK => resp(self.socket.clone(), self.route_get_block().await).await,
            ROUTE_GET_DIFFICULTY => {
                resp(self.socket.clone(), self.route_get_difficulty().await).await
            }
            ROUTE_GET_HIGHEST_BLOCK => {
                resp(self.socket.clone(), self.route_get_highest_block().await).await
            }
            ROUTE_ADD_BLOCK => resp(self.socket.clone(), self.route_add_block().await).await,
            _ => {
                log::debug!("unknown command: {}", command);

                send_err(self.socket.clone(), ServerError::UnknownCommand)
                    .await
                    .unwrap();

                false
            }
        }
    }

    async fn route_get_block(&mut self) -> ServerResult<Block> {
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

        Ok(block.clone())
    }

    async fn route_get_difficulty(&mut self) -> ServerResult<f64> {
        let node: tokio::sync::MutexGuard<'_, Node> = self.node.lock().await;

        let difficulty = node.get_difficulty();

        Ok(difficulty)
    }

    async fn route_get_highest_block(&mut self) -> ServerResult<HighestBlockResponse> {
        let node: tokio::sync::MutexGuard<'_, Node> = self.node.lock().await;

        let highest_block = node.get_highest_block();
        let height = node.get_height();
        let difficulty = node.get_difficulty();

        Ok(HighestBlockResponse {
            block: highest_block.clone(),
            height,
            difficulty,
        })
    }

    async fn route_add_block(&mut self) -> ServerResult<HighestBlockResponse> {
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
        let difficulty = node.get_difficulty();

        Ok(HighestBlockResponse {
            block,
            height,
            difficulty,
        })
    }
}

async fn resp<T>(socket: Arc<Mutex<TcpStream>>, data: ServerResult<T>) -> bool
where
    T: serde::Serialize,
{
    send_response(socket, &data).await.unwrap();

    data.is_ok()
}
