use sea_orm::{DbErr, DeriveMigrationName, sea_query::Table};
use sea_orm_migration::prelude::*;

use crate::identifiers::{job_instances::JobInstance, job_steps::JobStep, step_logs::StepLogs};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StepLogs::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(StepLogs::Id).uuid().primary_key())
                    .col(ColumnDef::new(StepLogs::JobInstanceId).uuid().not_null())
                    .col(ColumnDef::new(StepLogs::StepId).uuid().not_null())
                    .col(
                        ColumnDef::new(StepLogs::StepOrder)
                            .integer()
                            .not_null()
                            .default(Expr::value(0)),
                    )
                    .col(
                        ColumnDef::new(StepLogs::Status)
                            .custom("job_status")
                            .not_null(),
                    )
                    .col(ColumnDef::new(StepLogs::Output).text().not_null())
                    .col(
                        ColumnDef::new(StepLogs::FinishedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StepLogs::StartedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(StepLogs::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_step_logs_job_instance_id")
                            .from(StepLogs::Table, StepLogs::JobInstanceId)
                            .to(JobInstance::Table, JobInstance::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_step_logs_step_id")
                            .from(StepLogs::Table, StepLogs::StepId)
                            .to(JobStep::Table, JobStep::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_step_logs_step_order")
                    .table(StepLogs::Table)
                    .col(StepLogs::StepOrder)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_step_logs_job_instance_id")
                    .table(StepLogs::Table)
                    .col(StepLogs::JobInstanceId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(StepLogs::Table).to_owned())
            .await
    }
}
