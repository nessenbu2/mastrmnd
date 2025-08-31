use tonic::{transport::Server, Request, Response, Status};

pub mod mastrmnd {
    tonic::include_proto!("mastrmnd");
}

use mastrmnd::{echo_server::{Echo, EchoServer}, RegisterRequest, RegisterResponse, GetStateRequest, GetStateResponse, ClientState as ProtoClientState, MndState};

#[derive(Clone)]
pub struct EchoService {
    tracker: super::tracker::Tracker,
    store: super::client_state::ClientStateStore,
}

#[tonic::async_trait]
impl Echo for EchoService {
    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<RegisterResponse>, Status> {
        // Extract before consuming request
        let client_name = super::tracker::extract_client_name(&request);
        self.tracker.record(client_name.clone());

        let msg = request.into_inner().message;

        // Update detailed state store
        self.store.record_register(client_name.clone(), msg.clone(), None);

        // Optionally log a snapshot size for visibility (avoid large log spam)
        let total = self.tracker.snapshot().get(&client_name).cloned().unwrap_or(0);
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

pub async fn start_server(addr: std::net::SocketAddr, tracker: super::tracker::Tracker, store: super::client_state::ClientStateStore) -> Result<(), Box<dyn std::error::Error>> {
    let svc = EchoService { tracker, store };
    Server::builder()
        .add_service(EchoServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
