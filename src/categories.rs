use crate::database::Database;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize)]
pub struct Category {
    id: Uuid,
    name: String,
}

#[derive(Deserialize)]
pub struct NewCategory {
    name: String,
}

pub fn router() -> Router<Database<Category>> {
    Router::new()
        .route("/", get(get_categories).post(create_category))
        .route("/{category_id}", get(get_category).delete(delete_category))
}

async fn get_category(
    State(db): State<Database<Category>>,
    Path(id): Path<Uuid>,
) -> Json<Category> {
    todo!()
}

async fn create_category(
    State(db): State<Database<Category>>,
    Json(category): Json<NewCategory>,
) -> (StatusCode, Json<Category>) {
    todo!()
}
async fn delete_category(State(db): State<Database<Category>>, Path(id): Path<Uuid>) -> StatusCode {
    todo!()
}

async fn get_categories(State(db): State<Database<Category>>) -> Json<Vec<Category>> {
    todo!()
}
