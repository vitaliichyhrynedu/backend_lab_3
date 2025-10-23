use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use sea_orm::DatabaseConnection;
use serde::Serialize;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(check_health))
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum AppStatus {
    Healthy,
    Unhealthy,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
enum ServiceStatus {
    Up,
    Down,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
struct Services {
    db: ServiceStatus,
}

impl Services {
    async fn now(state: AppState) -> Self {
        Self {
            db: check_db(&state.db).await,
        }
    }

    fn status(&self) -> AppStatus {
        let critical_services = [&self.db];

        let healthy = critical_services
            .iter()
            .all(|status| matches!(status, ServiceStatus::Up));

        if healthy {
            AppStatus::Healthy
        } else {
            AppStatus::Unhealthy
        }
    }

    fn into_health(self) -> Health {
        Health {
            status: self.status(),
            services: self,
        }
    }
}

#[derive(Serialize)]
struct Health {
    status: AppStatus,
    services: Services,
}

impl IntoResponse for Health {
    fn into_response(self) -> axum::response::Response {
        let status = match self.status {
            AppStatus::Healthy => StatusCode::OK,
            AppStatus::Unhealthy => StatusCode::SERVICE_UNAVAILABLE,
        };
        let body = Json(self);
        (status, body).into_response()
    }
}

async fn check_health(State(state): State<AppState>) -> Health {
    Services::now(state).await.into_health()
}

async fn check_db(db: &DatabaseConnection) -> ServiceStatus {
    match db.ping().await {
        Ok(_) => ServiceStatus::Up,
        Err(e) => {
            eprintln!("database ping failed: {e}");
            ServiceStatus::Down
        }
    }
}
