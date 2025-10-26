use std::{env, error::Error};

use sea_orm::{Database, DatabaseConnection};

pub async fn connect() -> Result<DatabaseConnection, Box<dyn Error>> {
    let db_url = env::var("DATABASE_URL")?;
    let db = Database::connect(db_url).await?;
    Ok(db)
}
