use anyhow::Context;
use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};

pub async fn establish_connection() -> Result<DatabaseConnection, anyhow::Error> {
    dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").context("Missing environment variable 'DATABASE_URL'")?;
    let db = Database::connect(database_url)
        .await
        .context("Failed to connect to the database")?;
    db.ping()
        .await
        .context("Database does not respond. Please check the database connection")?;
    Ok(db)
}
