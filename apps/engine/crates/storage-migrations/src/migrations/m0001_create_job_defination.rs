use sea_orm_migration::prelude::*;

use crate::identifiers::job_defination::JobDefination;

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(JobDefination::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(JobDefination::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(JobDefination::Name).string().not_null())
                    .col(ColumnDef::new(JobDefination::RepoUrl).string().not_null())
                    .col(
                        ColumnDef::new(JobDefination::DefaultBranch)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(JobDefination::Steps).json().not_null())
                    .col(
                        ColumnDef::new(JobDefination::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(JobDefination::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(JobDefination::Table).to_owned())
            .await
    }
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m0001_create_job_defination_table"
    }
}
