use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, error::AppError};
use entity::category;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_categories).post(create_category))
        .route("/{category_id}", get(get_category).delete(delete_category))
}

#[derive(Debug, Serialize, Deserialize)]
struct CategoryBody<T> {
    category: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct CategoriesBody<T> {
    categories: Vec<T>,
}

#[derive(Debug, Serialize)]
struct Category {
    id: Uuid,
    name: String,
}

impl From<category::Model> for Category {
    fn from(value: category::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Debug, Deserialize)]
struct CategoryCreate {
    name: String,
}

async fn get_category(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<CategoryBody<Category>>, AppError> {
    let category = category::Entity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError::NotFound)?
        .into();
    Ok(Json(CategoryBody { category }))
}

async fn create_category(
    State(AppState { db }): State<AppState>,
    Json(body): Json<CategoryBody<CategoryCreate>>,
) -> Result<(StatusCode, Json<CategoryBody<Category>>), AppError> {
    let category = category::ActiveModel {
        name: Set(body.category.name),
        ..Default::default()
    };
    let category = category.insert(&db).await?.into();
    Ok((StatusCode::CREATED, Json(CategoryBody { category })))
}

async fn delete_category(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let res = category::Entity::delete_by_id(id).exec(&db).await?;
    match res.rows_affected {
        0 => Err(AppError::NotFound),
        _ => Ok(StatusCode::NO_CONTENT),
    }
}

async fn get_categories(
    State(AppState { db }): State<AppState>,
) -> Result<Json<CategoriesBody<Category>>, AppError> {
    let categories = category::Entity::find()
        .all(&db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(Json(CategoriesBody { categories }))
}
