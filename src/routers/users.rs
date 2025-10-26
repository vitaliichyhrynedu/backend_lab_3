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
use entity::user;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_users).post(create_user))
        .route("/{user_id}", get(get_user).delete(delete_user))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserBody<T> {
    user: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UsersBody<T> {
    users: Vec<T>,
}

#[derive(Debug, Serialize)]
pub struct User {
    id: Uuid,
    name: String,
}

impl From<user::Model> for User {
    fn from(value: user::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserCreate {
    name: String,
}

pub async fn get_user(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserBody<User>>, AppError> {
    let user = user::Entity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError::NotFound)?
        .into();
    Ok(Json(UserBody { user }))
}

pub async fn create_user(
    State(AppState { db }): State<AppState>,
    Json(body): Json<UserBody<UserCreate>>,
) -> Result<(StatusCode, Json<UserBody<User>>), AppError> {
    let user = user::ActiveModel {
        name: Set(body.user.name),
        ..Default::default()
    };
    let user = user.insert(&db).await?.into();
    Ok((StatusCode::CREATED, Json(UserBody { user })))
}

pub async fn delete_user(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let res = user::Entity::delete_by_id(id).exec(&db).await?;
    match res.rows_affected {
        0 => Err(AppError::NotFound),
        _ => Ok(StatusCode::NO_CONTENT),
    }
}

pub async fn get_users(
    State(AppState { db }): State<AppState>,
) -> Result<Json<UsersBody<User>>, AppError> {
    let users = user::Entity::find()
        .all(&db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(Json(UsersBody { users }))
}
