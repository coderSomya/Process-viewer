use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::State, routing::get, Router};
use sysinfo::System;
use std::sync::Mutex;use std::sync::Arc;
#[tokio::main]
async fn main() {

    let router = Router::new()
    .route("/", get(root_get))
    .route("/api/cpus", get(cpus_get))
    .with_state(AppState {
        sys: Arc::new(Mutex::new(System::new())),
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7032").await.unwrap();
    axum::serve(listener, router).await;

}

#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>
}

async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse{

    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();

    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    Json(v)
}

async fn root_get(State(state): State<AppState>) -> &'static str{

    "Hello world..!"
}
