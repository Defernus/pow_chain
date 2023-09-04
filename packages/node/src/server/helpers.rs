use std::{borrow::BorrowMut, error::Error, fmt::Display, sync::Arc};

use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};

use crate::config::Config;

use super::error::ServerError;

pub async fn send_err(
    socket: Arc<Mutex<TcpStream>>,
    err: ServerError,
) -> Result<(), Box<dyn Error>> {
    let mut lock = socket.lock().await;
    let socket = lock.borrow_mut();

    let err = serde_json::to_string(&err).unwrap();

    socket
        .write_all(format!("ERROR\n{}\n", err).as_bytes())
        .await?;
    socket.flush().await?;

    Ok(())
}

pub async fn send_response(
    socket: Arc<Mutex<TcpStream>>,
    data: impl Display,
) -> Result<(), Box<dyn Error>> {
    let mut lock = socket.lock().await;
    let socket = lock.borrow_mut();

    socket.write_all(format!("OK\n{data}\n").as_bytes()).await?;
    socket.flush().await?;

    Ok(())
}

/// Increases connections count and checks if it's greater than max connections.
///
/// Returns `true` if max connections is reached.
pub async fn add_connection(
    socket: Arc<Mutex<TcpStream>>,
    connections: Arc<Mutex<usize>>,
    cfg: &Config,
) -> Result<bool, Box<dyn Error>> {
    let mut c = connections.lock().await;
    *c += 1;

    if cfg.max_connections != 0 && *c > cfg.max_connections {
        log::warn!("max connections reached");

        send_err(socket.clone(), ServerError::MaxConnectionsReached).await?;
        socket.lock().await.borrow_mut().shutdown().await?;

        return Ok(true);
    }

    Ok(false)
}
