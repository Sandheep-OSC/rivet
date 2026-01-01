use sea_orm::{DbBackend, DeriveMigrationName, prelude::*, sea_query::extension::postgres::Type};
use sea_orm_migration::{MigrationTrait, SchemaManager};
use storage::identifiers::job_status_db::JobStatusDb;

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
                        .as_enum("job_status")
                        .values([
                            JobStatusDb::Pending,
                            JobStatusDb::Running,
                            JobStatusDb::Success,
                            JobStatusDb::Error,
                        ])
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        if db.get_database_backend() == DbBackend::Postgres {
            manager
                .drop_type(Type::drop().name("job_status").to_owned())
                .await?;
        }

        Ok(())
    }
}
