mod app;
mod config;
mod server;
mod ssh;

use crate::config::AppConfig;
use crate::server::AppServer;

#[tokio::main]
async fn main() {
    let config = AppConfig::load().expect("Failed loading config");

    let mut server = AppServer::new();
    server.run(config).await.expect("Failed running server");
}
