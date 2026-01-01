use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum JobInstance {
    Table,
    Id,
    JobDefinationId,
    Status,
    TriggeredBy,
    CommitSha,
    StartedAt,
    FinishedAt,
    CreatedAt,
}
