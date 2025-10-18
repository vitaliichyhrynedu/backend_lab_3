use std::{collections::HashMap, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

pub type Database = Arc<RwLock<HashMap<Uuid, User>>>;

pub fn router() -> Router<Database> {
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/{user_id}", get(get_user).delete(delete_user))
}

pub fn database() -> Database {
    Arc::new(RwLock::new(HashMap::new()))
}

#[derive(Clone, Serialize)]
pub struct User {
    id: Uuid,
    username: String,
}

#[derive(Deserialize)]
struct NewUser {
    username: String,
}

async fn get_user(State(db): State<Database>, Path(id): Path<Uuid>) -> Json<User> {
    todo!()
}

async fn create_user(
    State(db): State<Database>,
    Json(user): Json<NewUser>,
) -> (StatusCode, Json<User>) {
    todo!()
}

async fn delete_user(State(db): State<Database>, Path(id): Path<Uuid>) -> StatusCode {
    todo!()
}

async fn get_users(State(db): State<Database>) -> Json<Vec<User>> {
    todo!()
}
