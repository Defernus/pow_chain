use std::{error::Error, sync::Arc};

use tokio::{
    net::TcpListener,
    sync::{mpsc, Mutex},
};

use crate::{
    config::Config,
    node::Node,
    server::{
        error::ServerError,
        helpers::{add_connection, send_err},
        router::Router,
    },
};

pub mod error;
pub(self) mod helpers;
pub(self) mod router;

pub struct Server {
    cfg: Arc<Config>,
    listener: TcpListener,
    node: Arc<Mutex<Node>>,
}

impl Server {
    pub async fn new(cfg: Config, node: Node) -> Result<Self, Box<dyn Error>> {
        let addr = format!("0.0.0.0:{}", cfg.port);

        let listener = TcpListener::bind(addr).await?;

        Ok(Self {
            listener,
            cfg: Arc::new(cfg),
            node: Arc::new(Mutex::new(node)),
        })
    }

    // TODO add graceful shutdown
    pub async fn listen(&mut self) -> Result<Self, Box<dyn Error>> {
        log::info!("listening on: {}", self.listener.local_addr()?);

        let connections: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

        loop {
            let (socket, addr) = self.listener.accept().await?;

            log::debug!("accepted connection from: {}", addr);

            let socket = Arc::new(Mutex::new(socket));
            if add_connection(socket.clone(), connections.clone(), &self.cfg).await? {
                continue;
            }

            let connections = connections.clone();
            let max_body_size = self.cfg.max_body_size;
            let node = self.node.clone();
            let cfg = self.cfg.clone();

            tokio::spawn(async move {
                let (tx, rx) = mpsc::channel::<Option<String>>(100);

                let mut router = Router::new(cfg.clone(), socket.clone(), rx, node.clone());
                tokio::spawn(async move { router.run().await });

                let mut buf = vec![0; max_body_size + 1];
                loop {
                    if tx.is_closed() {
                        break;
                    }

                    let size = {
                        match socket.lock().await.try_read(&mut buf) {
                            Ok(n) => n,
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                            Err(e) => {
                                log::error!("failed to read from socket: {:?}", e);
                                break;
                            }
                        }
                    };

                    if size == 0 {
                        break;
                    }

                    if size > max_body_size {
                        log::warn!("max body size reached");
                        *connections.lock().await -= 1;

                        send_err(socket.clone(), ServerError::MaxBodySizeReached)
                            .await
                            .unwrap();
                        break;
                    }

                    let body = String::from_utf8_lossy(&buf[..size]).to_string();
                    let lines = body.split('\n').filter(|s| !s.is_empty());

                    for l in lines {
                        tx.send(Some(l.to_string()))
                            .await
                            .map_err(|e| format!("{e}"))
                            .unwrap();
                    }
                }

                log::debug!("closing connection from: {}", addr);
                if !tx.is_closed() {
                    tx.send(None).await.unwrap();
                }
                *connections.lock().await -= 1;
            });
        }
    }
}
