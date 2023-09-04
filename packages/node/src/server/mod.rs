use std::error::Error;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::config::Config;

pub struct Server {
    listener: TcpListener,
    cfg: Config,
}

impl Server {
    pub async fn new(cfg: Config) -> Result<Self, Box<dyn Error>> {
        let addr = format!("0.0.0.0:{}", cfg.port);

        let listener = TcpListener::bind(addr).await?;

        Ok(Self { listener, cfg })
    }

    pub async fn listen(&mut self) -> Result<Self, Box<dyn Error>> {
        loop {
            let (mut socket, addr) = self.listener.accept().await?;

            log::debug!("accepted connection from: {}", addr);

            tokio::spawn(async move {
                let mut buf = vec![0; 1024];

                loop {
                    let n = match socket.read(&mut buf).await {
                        Ok(n) => n,
                        Err(e) => {
                            log::error!("failed to read from socket: {:?}", e);
                            break;
                        }
                    };

                    if n == 0 {
                        break;
                    }

                    match socket.write_all(&buf[0..n]).await {
                        Ok(_) => (),
                        Err(e) => {
                            log::error!("failed to write to socket: {:?}", e);
                            break;
                        }
                    };
                }

                log::debug!("closing connection from: {}", addr);
            });
        }
    }
}
