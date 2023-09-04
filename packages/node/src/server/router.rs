use std::sync::Arc;

use tokio::{
    net::TcpStream,
    sync::{mpsc, Mutex},
};

use crate::server::helpers::send_response;

pub async fn start_router(socket: Arc<Mutex<TcpStream>>, mut rx: mpsc::Receiver<Option<String>>) {
    while let Some(msg) = next_msg(&mut rx).await {
        log::debug!("received message: {}", msg);

        send_response(socket.clone(), msg).await.unwrap();
    }

    rx.close();
}

async fn next_msg(rx: &mut mpsc::Receiver<Option<String>>) -> Option<String> {
    rx.recv().await.unwrap()
}
