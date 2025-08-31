use std::net::SocketAddr;

mod mastrmnd;
mod http_server;
mod tracker;
mod client_state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // gRPC server address
    let grpc_addr: SocketAddr = std::env::var("SERVER_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:50051".to_string())
        .parse()?;
    // HTTP server address
    let http_addr: SocketAddr = std::env::var("HTTP_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string())
        .parse()?;

    println!("Starting servers: gRPC={} HTTP={}", grpc_addr, http_addr);

    // Run both servers concurrently; if either fails, return the error
    let tracker = tracker::Tracker::new();
    let store = client_state::ClientStateStore::new();
    let grpc = mastrmnd::start_server(grpc_addr, tracker.clone(), store.clone());
    let http = http_server::start_http(http_addr, tracker.clone(), store.clone());
    tokio::try_join!(grpc, http)?;
    Ok(())
}
