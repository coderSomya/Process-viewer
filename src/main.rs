use axum::{routing::get, Router};


#[tokio::main]
async fn main() {

    let router = Router::new()
    .route("/", get(root_get));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7032").await.unwrap();
    axum::serve(listener, router).await.unwrap();

}

async fn root_get() -> &'static str{
    "Hi from Axum"
}
