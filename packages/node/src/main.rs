use std::error::Error;

use pow_chain_node::{config::Config, server::Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::new();

    let mut server = Server::new(cfg).await?;

    server.listen().await?;

    Ok(())
}
