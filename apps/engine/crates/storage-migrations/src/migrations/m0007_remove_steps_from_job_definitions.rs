use sea_orm::{DeriveMigrationName, sea_query::Table};
use sea_orm_migration::prelude::*;

use crate::identifiers::job_defination::JobDefination;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(JobDefination::Table)
                    .drop_column(JobDefination::Steps)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(JobDefination::Table)
                    .add_column(ColumnDef::new(JobDefination::Steps).json().not_null())
                    .to_owned(),
            )
            .await
    }
}
