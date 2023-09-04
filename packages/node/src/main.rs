use std::error::Error;

use pow_core::update_log_level::update_log_level;
use pow_node::{config::Config, server::Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cfg = Config::new();

    update_log_level();

    let mut server = Server::new(&cfg).await?;

    server.listen(&cfg).await?;

    Ok(())
}
