use std::net::SocketAddr;
use axum::{routing::get, routing::put, Router, response::IntoResponse, extract::{Path, State}, http::{StatusCode, HeaderValue, Method}, Json};
use axum::response::Response;
use axum::body::Body;
use tower_http::cors::{Any, CorsLayer};
use crate::library::Song;

#[derive(Clone)]
struct AppState {
    store: super::client_state::ClientStateStore,
    library: super::library::Library,
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

async fn toggle_state(Path(name) : Path<String>, State(state): State<AppState>)  -> impl IntoResponse {
    println!("Toggling client state: {}", name);
    state.store.toggle_state(name);
    StatusCode::OK
}

async fn inc_count(Path(name) : Path<String>, State(state): State<AppState> )  -> impl IntoResponse {
    println!("Incrementing client named: {}", name);
    state.store.inc(name);
    StatusCode::OK
}

async fn get_client_state(Path(name) : Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    println!("Getting state for client named: {}", name);
    let client_state = state.store.get_state(name);
    let body = serde_json::to_string(&client_state).unwrap_or_else(|_| "{}".to_string());
    Response::builder()
        .status(StatusCode::OK)
        .header(axum::http::header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(Body::from(body))
        .unwrap()
}

async fn get_library(State(state): State<AppState>) -> impl IntoResponse {
    println!("Getting library");
    let state = serde_json::to_string(&state.library.get_state())
        .unwrap_or_else(|_| "{}".to_string());
    Response::builder()
        .status(StatusCode::OK)
        .header(axum::http::header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(Body::from(state))
        .unwrap()
}

async fn play_song(Path(name) : Path<String>, State(state): State<AppState>, Json(song): Json<Song>) -> impl IntoResponse {
    println!("Playing song for client named: {}, song: {}", name, song.name);
    state.store.play_song(name, song);
    StatusCode::OK
}

pub async fn start_http(addr: SocketAddr, store: super::client_state::ClientStateStore, library: super::library::Library) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState { store, library };

    let app = Router::new()
        .route("/", get(root))
        .route("/clients", get(get_clients))
        .route("/clients/toggle/{name}", get(toggle_state))
        .route("/clients/inc/{name}", get(inc_count))
        .route("/clients/state/{name}", get(get_client_state))
        .route("/clients/play/{name}", put(play_song))
        .route("/library", get(get_library))
        .with_state(state)
        .layer(cors_layer());

    println!("HTTP server (axum) listening on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}
