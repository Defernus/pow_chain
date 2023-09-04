use std::{error::Error, fmt::Display, sync::Arc};

use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};

use crate::config::Config;

pub async fn send_err(socket: &mut TcpStream, err: impl Display) -> Result<(), Box<dyn Error>> {
    socket
        .write_all(format!("ERROR\n{err}\n").as_bytes())
        .await?;
    socket.flush().await?;

    Ok(())
}

pub async fn send_response(
    socket: &mut TcpStream,
    data: impl Display,
) -> Result<(), Box<dyn Error>> {
    socket.write_all(format!("OK\n{data}\n").as_bytes()).await?;
    socket.flush().await?;

    Ok(())
}

/// Increases connections count and checks if it's greater than max connections.
///
/// Returns `true` if max connections is reached.
pub async fn add_connection(
    socket: &mut TcpStream,
    connections: Arc<Mutex<usize>>,
    cfg: &Config,
) -> Result<bool, Box<dyn Error>> {
    let mut c = connections.lock().await;
    *c += 1;

    if cfg.max_connections != 0 && *c > cfg.max_connections {
        log::warn!("max connections reached");

        send_err(socket, "max connections reached").await?;
        socket.shutdown().await?;

        return Ok(true);
    }

    Ok(false)
}
