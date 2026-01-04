use sea_orm::{Database, DatabaseConnection};
use tracing::info;

pub async fn init_db(database_url: &str) -> DatabaseConnection {
    let db = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");

    info!("Connected to the database");

    db
}
