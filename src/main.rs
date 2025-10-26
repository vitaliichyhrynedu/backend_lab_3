mod database;
mod routers;

use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
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
    let db = match database::connect().await {
        Ok(db) => {
            eprintln!("database connection established");
            db
        }
        Err(e) => {
            eprintln!("database connection failed: {}", e);
            std::process::exit(1);
        }
    };
    Migrator::up(&db, None).await.unwrap();

    let state = AppState { db: db };
    let router = routers::router().with_state(state);

    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let addr = format!("{}:{}", host, port);

    eprintln!("starting server");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    eprintln!("listening on http://{}", &addr);
    axum::serve(listener, router).await.unwrap();
}
