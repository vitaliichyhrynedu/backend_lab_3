use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use chrono::NaiveDateTime;
use entity::{category, record, user};
use rust_decimal::Decimal;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use tokio::try_join;
use uuid::Uuid;

use crate::{AppState, error::AppError};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_records).post(create_record))
        .route("/{record_id}", get(get_record).delete(delete_record))
}

#[derive(Debug, Serialize, Deserialize)]
struct RecordBody<T> {
    record: T,
}

#[derive(Debug, Serialize)]
struct RecordsBody<T> {
    records: Vec<T>,
}

#[derive(Clone, Serialize)]
pub struct Record {
    id: Uuid,
    user_id: Uuid,
    category_id: Uuid,
    created_at: NaiveDateTime,
    sum: Decimal,
}

impl From<record::Model> for Record {
    fn from(value: record::Model) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            category_id: value.category_id,
            created_at: value.created_at,
            sum: value.sum,
        }
    }
}

#[derive(Deserialize)]
pub struct RecordCreate {
    user_id: Uuid,
    category_id: Uuid,
    sum: Decimal,
}

impl RecordCreate {
    async fn validate(&self, db: &DatabaseConnection) -> Result<(), AppError> {
        let mut errors = Vec::new();

        let (user, category) = try_join!(
            user::Entity::find_by_id(self.user_id).one(db),
            category::Entity::find_by_id(self.category_id).one(db)
        )?;

        if user.is_none() {
            errors.push(("user_id", "user doesn't exist"));
        }

        if category.is_none() {
            errors.push(("category_id", "category doesn't exist"));
        }

        if self.sum <= Decimal::ZERO {
            errors.push(("sum", "sum is not positive"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(AppError::unprocessable_entity(errors))
        }
    }
}

#[derive(Deserialize)]
pub struct RecordFilterParams {
    user_id: Option<Uuid>,
    category_id: Option<Uuid>,
}

async fn get_record(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<RecordBody<Record>>, AppError> {
    let record = record::Entity::find_by_id(id)
        .one(&db)
        .await?
        .ok_or(AppError::NotFound)?
        .into();
    Ok(Json(RecordBody { record }))
}

async fn create_record(
    State(AppState { db }): State<AppState>,
    Json(body): Json<RecordBody<RecordCreate>>,
) -> Result<(StatusCode, Json<RecordBody<Record>>), AppError> {
    body.record.validate(&db).await?;
    let record = record::ActiveModel {
        user_id: Set(body.record.user_id),
        category_id: Set(body.record.category_id),
        sum: Set(body.record.sum),
        ..Default::default()
    };
    let record = record.insert(&db).await?.into();
    Ok((StatusCode::CREATED, Json(RecordBody { record })))
}

async fn delete_record(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let res = record::Entity::delete_by_id(id).exec(&db).await?;
    match res.rows_affected {
        0 => Err(AppError::NotFound),
        _ => Ok(StatusCode::NO_CONTENT),
    }
}

async fn get_records(
    State(AppState { db }): State<AppState>,
    Query(params): Query<RecordFilterParams>,
) -> Result<Json<RecordsBody<Record>>, AppError> {
    let mut query = record::Entity::find();
    if let Some(user_id) = params.user_id {
        query = query.filter(record::Column::UserId.eq(user_id));
    }
    if let Some(category_id) = params.category_id {
        query = query.filter(record::Column::CategoryId.eq(category_id));
    }
    let records = query.all(&db).await?.into_iter().map(Into::into).collect();
    Ok(Json(RecordsBody { records }))
}
