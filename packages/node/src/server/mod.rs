use std::{error::Error, sync::Arc};

use tokio::{
    io::AsyncReadExt,
    net::TcpListener,
    sync::{mpsc, Mutex},
};

use crate::{
    config::Config,
    server::{
        helpers::{add_connection, send_err},
        router::start_router,
    },
};

pub(self) mod helpers;
pub(self) mod router;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new(cfg: &Config) -> Result<Self, Box<dyn Error>> {
        let addr = format!("0.0.0.0:{}", cfg.port);

        let listener = TcpListener::bind(addr).await?;

        Ok(Self { listener })
    }

    // TODO add graceful shutdown
    pub async fn listen(&mut self, cfg: &Config) -> Result<Self, Box<dyn Error>> {
        log::info!("listening on: {}", self.listener.local_addr()?);

        let connections: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

        loop {
            let (socket, addr) = self.listener.accept().await?;

            log::debug!("accepted connection from: {}", addr);

            let socket = Arc::new(Mutex::new(socket));
            if add_connection(socket.clone(), connections.clone(), cfg).await? {
                continue;
            }

            let max_body_size = cfg.max_body_size;

            let connections = connections.clone();

            tokio::spawn(async move {
                let (tx, rx) = mpsc::channel::<Option<String>>(100);

                let router_socket = socket.clone();
                tokio::spawn(async move {
                    start_router(router_socket, rx).await;
                });

                let mut buf = vec![0; max_body_size + 1];
                loop {
                    let size = {
                        match socket.lock().await.read(&mut buf).await {
                            Ok(n) => n,
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

                        send_err(socket.clone(), "max body size reached")
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
                tx.send(None).await.unwrap();
                *connections.lock().await -= 1;
            });
        }
    }
}
