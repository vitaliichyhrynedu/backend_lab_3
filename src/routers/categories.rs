use crate::database::Database;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn router() -> Router<Database<Category>> {
    Router::new()
        .route("/", get(get_categories).post(create_category))
        .route("/{category_id}", get(get_category).delete(delete_category))
}

#[derive(Clone, Serialize)]
pub struct Category {
    id: Uuid,
    name: String,
}

#[derive(Deserialize)]
pub struct NewCategory {
    name: String,
}

async fn get_category(
    State(db): State<Database<Category>>,
    Path(id): Path<Uuid>,
) -> Json<Category> {
    let db = db.read().await;
    let category = db.get(&id).unwrap().clone();
    Json(category)
}

async fn create_category(
    State(db): State<Database<Category>>,
    Json(category): Json<NewCategory>,
) -> (StatusCode, Json<Category>) {
    let mut db = db.write().await;
    let category = Category {
        id: Uuid::new_v4(),
        name: category.name,
    };
    db.insert(category.id, category.clone());
    (StatusCode::CREATED, Json(category))
}

async fn delete_category(State(db): State<Database<Category>>, Path(id): Path<Uuid>) -> StatusCode {
    let mut db = db.write().await;
    db.remove(&id);
    StatusCode::NO_CONTENT
}

async fn get_categories(State(db): State<Database<Category>>) -> Json<Vec<Category>> {
    let db = db.read().await;
    Json(db.values().cloned().collect())
}
