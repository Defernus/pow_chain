use std::error::Error;

use pow_core::update_log_level::update_log_level;
use pow_node::{config::Config, node::Node, server::Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::new();

    update_log_level();

    let node = Node::new("Initial block");

    let mut server = Server::new(cfg, node).await?;

    server.listen().await?;

    Ok(())
}
