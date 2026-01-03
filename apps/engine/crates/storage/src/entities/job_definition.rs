use sea_orm::{DeriveEntityModel, prelude::*};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "job_defination")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub name: String,
    pub repo_url: String,
    pub default_branch: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::job_instance::Entity")]
    JobInstances,

    #[sea_orm(has_many = "super::job_steps::Entity")]
    JobSteps,
}

impl Related<super::job_instance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JobInstances.def()
    }
}

impl Related<super::job_steps::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JobSteps.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
