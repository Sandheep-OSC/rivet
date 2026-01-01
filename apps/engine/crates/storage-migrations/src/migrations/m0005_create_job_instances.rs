use sea_orm_migration::prelude::*;

use crate::identifiers::{job_defination::JobDefination, job_instances::JobInstance};

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
                    .col(
                        ColumnDef::new(JobInstance::Status)
                            .custom("job_status")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JobInstance::TriggeredBy)
                            .custom("job_trigger")
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JobInstance::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JobInstance::StartedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JobInstance::FinishedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(JobInstance::Table, JobInstance::JobDefinationId)
                            .to(JobDefination::Table, JobDefination::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
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
