use axum::{Router, routing::get};

use crate::AppState;

// pub mod categories;
pub mod health;
// pub mod records;
pub mod users;

pub fn router() -> Router<AppState> {
    let health_router = health::router();
    let user_router = users::router();
    // let category_router = categories::router();
    // let record_router = records::router();

    let router = Router::new()
        .route("/", get(root))
        .nest("/health", health_router)
        .nest("/users", user_router);
    // .nest("/categories", category_router)
    // .nest("/records", record_router)

    router
}

async fn root() -> &'static str {
    "Welcome to the expense tracker!"
}
