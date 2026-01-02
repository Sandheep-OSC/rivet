use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::{env, sync::Once};
use tokio::sync::OnceCell;

static INIT: Once = Once::new();
static DATABASE_URL: OnceCell<String> = OnceCell::const_new();

pub struct TestContext {
    pub db: DatabaseConnection,
}

impl TestContext {
    pub async fn new() -> Result<Self, DbErr> {
        INIT.call_once(|| {
            // Load .env file once
            dotenv().ok();

            // Ensure DATABASE_URL exists
            if env::var("DATABASE_URL").is_err() {
                panic!("DATABASE_URL must be set in the environment or .env file");
            }
        });

        let db_url = DATABASE_URL
            .get_or_init(|| async { env::var("DATABASE_URL").expect("DATABASE_URL must be set") })
            .await;

        let db = Database::connect(db_url).await?;

        Ok(Self { db })
    }

    /// Insert test fixture data
    pub async fn insert_fixture_data(&self) -> Result<(), DbErr> {
        // Example:
        // use crate::entity::user;
        // let user = user::ActiveModel {
        //     username: Set("test_user".to_string()),
        //     email: Set("test@example.com".to_string()),
        //     ..Default::default()
        // };
        // user.insert(&self.db).await?;
        Ok(())
    }

    /// Load test state
    pub async fn load_state(&self) -> Result<(), DbErr> {
        // Optional: preload some data
        Ok(())
    }
}
