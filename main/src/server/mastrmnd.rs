use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

pub mod mastrmnd {
    tonic::include_proto!("mastrmnd");
}

use mastrmnd::{echo_server::{Echo, EchoServer}, RegisterRequest, RegisterResponse, GetStateRequest, GetStateResponse, ClientState as ProtoClientState, MndState};

#[derive(Clone)]
pub struct EchoService {
    store: super::client_state::ClientStateStore,
}

#[tonic::async_trait]
impl Echo for EchoService {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        // Extract before consuming request
        let client_name = extract_client_name(&request);

        let msg = request.into_inner().message;

        // Update detailed state store
        self.store.record_register(client_name.clone(), msg.clone(), None);

        // Optionally log a snapshot size for visibility (avoid large log spam)
        let total = self.store.snapshot_counts().get(&client_name).cloned().unwrap_or(0);
        println!("gRPC Echo.register from {} -> count={}", client_name, total);

        Ok(Response::new(RegisterResponse { message: msg }))
    }

    async fn get_state(&self, _request: Request<GetStateRequest>) -> Result<Response<GetStateResponse>, Status> {
        // Build response from store snapshot
        let items = self.store.snapshot();
        let states: Vec<ProtoClientState> = items.into_iter().map(|s| {
            let mnd_state = match s.state {
                super::client_state::State::Idle => MndState::Idle as i32,
                super::client_state::State::Playing => MndState::Playing as i32,
            };
            ProtoClientState {
                name: s.name,
                call_count: s.call_count,
                last_seen_secs: s.last_seen_secs,
                last_message: s.last_message,
                state: mnd_state,
            }
        }).collect();
        Ok(Response::new(GetStateResponse { states }))
    }
}

pub fn extract_client_name<T>(req: &tonic::Request<T>) -> String {
    // Print all metadata key/value pairs for visibility
    // Note: avoid panics; bound output sizes
    let md = req.metadata();
    if !md.is_empty() {
        // Collect printable lines
        let mut lines: Vec<String> = Vec::new();
        for kv in md.iter() {
            use tonic::metadata::{KeyAndValueRef, ValueRef};
            match kv {
                KeyAndValueRef::Ascii(k, v) => {
                    let name = k.as_str();
                    match v.to_str() {
                        Ok(s) => {
                            let s_trunc = if s.len() > 256 { &s[..256] } else { s };
                            lines.push(format!("{}: {}", name, s_trunc));
                        }
                        Err(_) => {
                            // Should not happen for Ascii, but be safe
                            let bytes = v.as_bytes();
                            let n = bytes.len().min(32);
                            let hex: String = bytes[..n].iter().map(|b| format!("{:02x}", b)).collect();
                            let more = if bytes.len() > n { "…" } else { "" };
                            lines.push(format!("{}: <ascii-bytes {}: {}{}>", name, bytes.len(), hex, more));
                        }
                    }
                }
                KeyAndValueRef::Binary(k, v) => {
                    let name = k.as_str();
                    let bytes = v.as_encoded_bytes();
                    let n = bytes.len().min(32);
                    let hex: String = bytes[..n].iter().map(|b| format!("{:02x}", b)).collect();
                    let more = if bytes.len() > n { "…" } else { "" };
                    lines.push(format!("{}: <binary {} bytes: {}{}>", name, bytes.len(), hex, more));
                }
            }
        }
        if !lines.is_empty() {
            println!("gRPC metadata ({} entries):\n{}", lines.len(), lines.join("\n"));
        }
    }

    // Prefer explicit metadata header x-client-name, then x-client-id
    if let Some(val) = req.metadata().get("x-client-name") {
        if let Ok(s) = val.to_str() { return format!("name:{}", s); }
    }
    if let Some(val) = req.metadata().get("x-client-id") {
        if let Ok(s) = val.to_str() { return format!("name:{}", s); }
    }
    // Try to get peer addr from extensions (set by Tonic/Hyper)
    if let Some(peer) = req.extensions().get::<SocketAddr>() {
        return format!("peer:{}", peer);
    }
    // Fallback to empty/unknown
    "unknown".to_string()
}

pub async fn start_server(addr: std::net::SocketAddr, store: super::client_state::ClientStateStore) -> Result<(), Box<dyn std::error::Error>> {
    let svc = EchoService { store };
    Server::builder()
        .add_service(EchoServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
