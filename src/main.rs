use crate::server::AppServer;

mod app;
mod server;
mod ssh;

#[tokio::main]
async fn main() {
    let mut server = AppServer::new();
    server.run().await.expect("Failed running server");
}
