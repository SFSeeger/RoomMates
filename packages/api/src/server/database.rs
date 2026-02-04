use anyhow::Context;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn establish_connection() -> Result<DatabaseConnection, anyhow::Error> {
    let _ = dotenvy::dotenv();
    let database_url =
        std::env::var("DATABASE_URL").context("Missing environment variable 'DATABASE_URL'")?;

    #[allow(unused_mut)]
    let mut options = ConnectOptions::new(database_url);
    #[cfg(not(debug_assertions))]
    options.sqlx_logging(false);

    let db = Database::connect(options)
        .await
        .context("Failed to connect to the database")?;
    db.ping()
        .await
        .context("Database does not respond. Please check the database connection")?;
    Ok(db)
}
