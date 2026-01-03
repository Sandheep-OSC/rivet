use sea_orm_migration::prelude::*;

use crate::identifiers::{job_defination::JobDefination, job_steps::JobStep};

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(JobStep::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(JobStep::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(JobStep::JobDefinationId).uuid().not_null())
                    .col(
                        ColumnDef::new(JobStep::StepOrder)
                            .integer()
                            .not_null()
                            .default(Expr::value(0)),
                    )
                    .col(ColumnDef::new(JobStep::Name).text().not_null())
                    .col(ColumnDef::new(JobStep::Command).string().not_null())
                    .col(ColumnDef::new(JobStep::WorkingDir).string())
                    .col(
                        ColumnDef::new(JobStep::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .index(
                        Index::create()
                            .name("idx_job_step_definition_order")
                            .col(JobStep::JobDefinationId)
                            .col(JobStep::StepOrder)
                            .unique(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("job_step_job_defination_fk")
                            .from(JobStep::Table, JobStep::JobDefinationId)
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
            .drop_table(Table::drop().table(JobStep::Table).to_owned())
            .await
    }
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m0002_create_job_step_table"
    }
}
