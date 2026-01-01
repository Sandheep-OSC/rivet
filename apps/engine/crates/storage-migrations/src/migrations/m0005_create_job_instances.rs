use sea_orm_migration::prelude::*;

use crate::identifiers::job_instances::JobInstance;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(JobInstance::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(JobInstance::Id)
                            .uuid()
                            .primary_key()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JobInstance::JobDefinationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(ColumnDef::new(JobInstance::CommitSha).text())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(JobInstance::Table).to_owned())
            .await
    }
}
