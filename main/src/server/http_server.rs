use std::net::SocketAddr;
use axum::{routing::get, Router, response::IntoResponse, extract::{Path, State},
           http::{StatusCode, HeaderValue, Method}};
use axum::response::Response;
use axum::body::Body;
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
struct AppState {
    store: super::client_state::ClientStateStore,
}

// Helper to build CORS layer similar to previous headers
fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any)
        .max_age(std::time::Duration::from_secs(600))
}

async fn root() -> impl IntoResponse {
    (StatusCode::OK, "hello from http server")
}

async fn get_clients(State(state): State<AppState>) -> impl IntoResponse {
    let snapshot = state.store.snapshot();
    let body = match serde_json::to_string(&snapshot) { Ok(s) => s, Err(_) => "{}".to_string() };
    Response::builder()
        .status(StatusCode::OK)
        .header(axum::http::header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(Body::from(body))
        .unwrap()
}

async fn get_clients_state(State(state): State<AppState>) -> impl IntoResponse {
    let detail = state.store.snapshot();
    let body = match serde_json::to_string(&detail) { Ok(s) => s, Err(_) => "[]".to_string() };
    Response::builder()
        .status(StatusCode::OK)
        .header(axum::http::header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(Body::from(body))
        .unwrap()
}

async fn inc_client(State(state): State<AppState>, Path((id)) : Path<(String)>) {
}

pub async fn start_http(addr: SocketAddr, store: super::client_state::ClientStateStore) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState { store };

    let app = Router::new()
        .route("/", get(root))
        .route("/clients", get(get_clients))
        .route("/clients/state", get(get_clients_state))
        .route("/clients/inc/{client_id}", get(inc_client))
        .with_state(state)
        .layer(cors_layer());

    println!("HTTP server (axum) listening on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}
