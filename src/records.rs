use std::fmt::Display;

use crate::database::Database;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub fn router() -> Router<Database<Record>> {
    Router::new()
        .route("/", get(get_records).post(create_record))
        .route("/{record_id}", get(get_record).delete(delete_record))
}

#[derive(Clone, Serialize)]
pub struct Record {
    id: Uuid,
    user_id: Uuid,
    category_id: Uuid,
    datetime: NaiveDateTime,
    sum: f64,
}

#[derive(Deserialize)]
pub struct NewRecord {
    user_id: Uuid,
    category_id: Uuid,
    sum: f64,
}

#[derive(Deserialize)]
pub struct RecordFilterParams {
    user_id: Option<Uuid>,
    category_id: Option<Uuid>,
}

enum RecordError {
    MissingFilterParams,
}

impl Display for RecordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordError::MissingFilterParams => {
                write!(f, "At least one filter parameter must be provided")
            }
        }
    }
}

impl IntoResponse for RecordError {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let status = match self {
            RecordError::MissingFilterParams => StatusCode::BAD_REQUEST,
        };

        let body = Json(ErrorResponse {
            message: self.to_string(),
        });

        (status, body).into_response()
    }
}

async fn get_record(State(db): State<Database<Record>>, Path(id): Path<Uuid>) -> Json<Record> {
    let db = db.read().await;
    let record = db.get(&id).unwrap().clone();
    Json(record)
}

async fn create_record(
    State(db): State<Database<Record>>,
    Json(record): Json<NewRecord>,
) -> (StatusCode, Json<Record>) {
    let mut db = db.write().await;
    let record = Record {
        id: Uuid::new_v4(),
        user_id: record.user_id,
        category_id: record.category_id,
        datetime: Utc::now().naive_utc(),
        sum: record.sum,
    };
    db.insert(record.id, record.clone());
    (StatusCode::CREATED, Json(record))
}

async fn delete_record(State(db): State<Database<Record>>, Path(id): Path<Uuid>) -> StatusCode {
    let mut db = db.write().await;
    db.remove(&id);
    StatusCode::NO_CONTENT
}

async fn get_records(
    State(db): State<Database<Record>>,
    Query(params): Query<RecordFilterParams>,
) -> Result<Json<Vec<Record>>, RecordError> {
    let (user_id, category_id) = (params.user_id, params.category_id);
    if user_id.is_none() && category_id.is_none() {
        return Err(RecordError::MissingFilterParams);
    }

    let db = db.read().await;
    let records = db
        .values()
        .filter(|record| user_id.map_or(true, |user_id| record.user_id == user_id))
        .filter(|record| category_id.map_or(true, |category_id| record.category_id == category_id))
        .cloned()
        .collect();

    Ok(Json(records))
}
