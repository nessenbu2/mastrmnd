use tonic::transport::Channel;
use tonic::Request;
use tonic::metadata::MetadataValue;

pub mod mastrmnd {
    tonic::include_proto!("mastrmnd");
}

use mastrmnd::echo_client::EchoClient;
use mastrmnd::{RegisterRequest, GetStateRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Endpoint to connect to (must be prefixed with http://)
    let endpoint = std::env::var("ECHO_ADDR")
        .unwrap_or_else(|_| "http://127.0.0.1:50051".to_string());

    let channel: Channel = Channel::from_shared(endpoint.clone())?
        .connect()
        .await?;

    let mut client = EchoClient::new(channel);

    // Determine client name: prefer CLIENT_NAME, then CLIENT_ID; else generate random
    let client_name = std::env::var("CLIENT_NAME")
        .ok()
        .filter(|s| !s.is_empty())
        .or_else(|| std::env::var("CLIENT_ID").ok().filter(|s| !s.is_empty()))
        .unwrap_or_else(|| {
            // Simple random fallback without extra deps
            use std::time::{SystemTime, UNIX_EPOCH};
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
            let pid = std::process::id();
            format!("auto-{}-{}", pid, ts)
        });
    println!("Using client_name={}", client_name);

    let msg = std::env::args().nth(1).unwrap_or_else(|| "hello from client".to_string());
    let mut request = Request::new(RegisterRequest { message: msg });
    // Attach x-client-name metadata (and x-client-id for backward compatibility)
    if let Ok(val) = MetadataValue::try_from(client_name.as_str()) {
        request.metadata_mut().insert("x-client-name", val.clone());
        request.metadata_mut().insert("x-client-id", val);
    } else {
        eprintln!("Warning: CLIENT_NAME/CLIENT_ID contains invalid characters for gRPC metadata; skipping headers");
    }

    match client.register(request).await {
        Ok(response) => {
            println!("{}", response.into_inner().message);
        }
        Err(status) => {
            eprintln!("gRPC error: {}", status);
        }
    }

    // Periodically poll GetState
    let interval_ms: u64 = std::env::var("STATE_POLL_MS").ok().and_then(|s| s.parse().ok()).unwrap_or(2000);
    println!("Starting state poll every {} ms", interval_ms);
    loop {
        let req = Request::new(GetStateRequest{});
        match client.get_state(req).await {
            Ok(resp) => {
                let states = resp.into_inner().states;
                let summary: Vec<String> = states.iter().map(|s| format!("{}: calls={} state={}", s.name, s.call_count, s.state)).collect();
                println!("State: [{}]", summary.join(", "));
            }
            Err(err) => {
                eprintln!("GetState error: {}", err);
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(interval_ms)).await;
    }
}
