use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::info;

#[derive(Clone)]
pub struct Db {
    pub conn: DatabaseConnection,
}

impl Db {
    pub async fn init(database_url: &str) -> Result<Self, DbErr> {
        let conn = Database::connect(database_url).await?;

        info!("✅ Database connection established");

        Ok(Self { conn })
    }
}
