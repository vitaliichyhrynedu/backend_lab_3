use axum::{Router, routing::get};
use std::env;

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(root));

    let port = env::var("PORT").expect("PORT environment variable must be set");
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to the expense tracker!"
}
