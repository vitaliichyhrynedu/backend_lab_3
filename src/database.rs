use std::{env, error::Error};

use sea_orm::{Database, DatabaseConnection};

pub async fn create_connection() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db_host = env::var("DB_HOST")?;
    let db_user = env::var("DB_USER")?;
    let db_pass = env::var("DB_PASS")?;
    let db_port = env::var("DB_PORT")?;

    let db_url = format!("postgres://{db_user}:{db_pass}@{db_host}:{db_port}/{db_user}");

    let db = Database::connect(db_url).await?;

    Ok(db)
}
