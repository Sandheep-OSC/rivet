use sea_orm::{DbBackend, DeriveMigrationName, prelude::*, sea_query::extension::postgres::Type};
use sea_orm_migration::{MigrationTrait, SchemaManager, async_trait};
use storage::identifiers::job_trigger_db::JobTriggerDb;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        if db.get_database_backend() == DbBackend::Postgres {
            manager
                .create_type(
                    Type::create()
                        .as_enum("job_trigger")
                        .values([
                            JobTriggerDb::WEBHOOK,
                            JobTriggerDb::MANUAL,
                            JobTriggerDb::CRON,
                        ])
                        .to_owned(),
                )
                .await?
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        if db.get_database_backend() == DbBackend::Postgres {
            manager
                .drop_type(Type::drop().name("job_trigger").to_owned())
                .await?;
        }

        Ok(())
    }
}
