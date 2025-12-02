use dotenv::dotenv;
use sea_orm::{Database, DatabaseConnection};

pub async fn establish_connection() -> DatabaseConnection {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(database_url)
        .await
        .expect("Failed to Connect to DB");
    db.ping().await.expect("DB does not respond to ping!");
    db
}
