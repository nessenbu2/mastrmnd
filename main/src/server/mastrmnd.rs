use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

pub mod mastrmnd {
    tonic::include_proto!("mastrmnd");
}

use mastrmnd::{echo_server::{Echo, EchoServer}, RegisterRequest, RegisterResponse, GetStateRequest, GetStateResponse, ClientState as ProtoClientState, MndState};

#[derive(Clone)]
pub struct MastrmndService {
    store: super::client_state::ClientStateStore,
    library: super::library::Library,
}

#[tonic::async_trait]
impl Echo for MastrmndService {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {

        // If the client actually set a name, use that, otherwise, pull the address out of the request
        let extensions = request.extensions().clone();
        let req = request.into_inner();
        let mut client_name = req.client_name;
        if client_name.is_empty() {
            if let Some(peer) = extensions.get::<SocketAddr>() {
                client_name = format!("peer:{}", peer);
            }
        }

        // Update detailed state store
        self.store.record_register(client_name.clone(), req.port);

        // Optionally log a snapshot size for visibility (avoid large log spam)
        let total = self.store.snapshot_counts().get(&client_name).cloned().unwrap_or(0);
        println!("gRPC register from {} -> count={}", client_name, total);

        Ok(Response::new(RegisterResponse { message: "ACCEPTED".to_string() }))
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
                state: mnd_state,
            }
        }).collect();
        Ok(Response::new(GetStateResponse { states }))
    }
}

pub async fn start_server(addr: SocketAddr, store: super::client_state::ClientStateStore, library: super::library::Library) -> Result<(), Box<dyn std::error::Error>> {
    let svc = MastrmndService { store, library };
    Server::builder()
        .add_service(EchoServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
