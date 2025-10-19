use axum::{Json, Router, routing::get};
use serde::Serialize;

pub fn router() -> Router {
    Router::new().route("/", get(health))
}

#[derive(Serialize)]
enum State {
    Up,
    Down,
}

#[derive(Serialize)]
struct Health {
    state: State,
}

async fn health() -> Json<Health> {
    Json(Health { state: State::Up })
}
