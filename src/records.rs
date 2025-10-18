use std::fmt::Display;

use crate::database::Database;
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
};
use chrono::NaiveDateTime;
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
    todo!()
}

async fn create_record(
    State(db): State<Database<Record>>,
    Json(record): Json<NewRecord>,
) -> (StatusCode, Json<Record>) {
    todo!()
}

async fn delete_record(State(db): State<Database<Record>>, Path(id): Path<Uuid>) -> StatusCode {
    todo!()
}

async fn get_records(
    State(db): State<Database<Record>>,
    Query(params): Query<RecordFilterParams>,
) -> Result<Json<Vec<Record>>, RecordError> {
    todo!()
}
