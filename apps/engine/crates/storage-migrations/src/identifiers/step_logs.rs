use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum StepLogs {
    Table,
    Id,
    JobInstanceId,
    StepId,
    StepOrder,
    Status,
    Output,
    StartedAt,
    FinishedAt,
    CreatedAt,
}
