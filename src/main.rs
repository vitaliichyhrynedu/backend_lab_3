// mod categories;
mod database;
mod health;
// mod records;
// mod users;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use std::env;

#[derive(Clone)]
struct AppState {
    db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    eprintln!("connecting to database");
    let db = match database::connection().await {
        Ok(db) => {
            eprintln!("database connection established");
            db
        }
        Err(e) => {
            eprintln!("database connection failed: {}", e);
            std::process::exit(1);
        }
    };

    let state = AppState { db: db };

    let health_router = health::router();
    // let user_router = users::router();
    // let category_router = categories::router();
    // let record_router = records::router();

    let router = Router::new()
        .route("/", get(root))
        .nest("/health", health_router)
        // .nest("/users", user_router)
        // .nest("/categories", category_router)
        // .nest("/records", record_router)
        .with_state(state);

    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let addr = format!("{}:{}", host, port);

    eprintln!("starting server");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    eprintln!("listening on http://{}", &addr);
    axum::serve(listener, router).await.unwrap();
}

async fn root() -> &'static str {
    "Welcome to the expense tracker!"
}
