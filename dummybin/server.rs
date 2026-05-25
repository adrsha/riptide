use std::sync::Arc;

use riptide::rt_server::RTServer;

#[tokio::main]
async fn main() {
    let server = RTServer::default();
    if let Err(result) = (server.run)(Arc::new(server)).await {
        result.log();
    }
}
