use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum JobDefination {
    Table,
    Id,
    Name,
    RepoUrl,
    DefaultBranch,
    Steps,
    CreatedAt,
    UpdatedAt,
}
