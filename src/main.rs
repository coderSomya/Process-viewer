use axum::{extract::State, routing::get, Router};
use sysinfo::System;
use std::sync::Mutex;use std::sync::Arc;
#[tokio::main]
async fn main() {

    let router = Router::new()
    .route("/", get(root_get))
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

async fn root_get(State(state): State<AppState>) -> String{
    use std::fmt::Write;


    let mut s = String::new();
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();

    for (i, cpu) in sys.cpus().iter().enumerate() {
        let i = i+1;
        let usage = cpu.cpu_usage();

        writeln!(&mut s, "CPU {i} {usage}%");
    }

    s
}

