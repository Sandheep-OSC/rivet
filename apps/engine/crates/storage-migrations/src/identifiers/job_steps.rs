use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum JobStep {
    Table,
    Id,
    JobDefinationId,
    StepOrder,
    Name,
    Command,
    WorkingDir,
    CreatedAt,
}
