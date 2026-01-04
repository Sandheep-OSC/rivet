use sea_orm::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "job_step")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub job_defination_id: Uuid,
    #[sea_orm(indexed)]
    pub step_order: i32,
    pub name: String,
    pub command: String,
    #[sea_orm(nullable)]
    pub working_dir: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Clone, Debug, EnumIter, DeriveRelation, PartialEq, Eq, Serialize)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::job_definition::Entity",
        from = "Column::JobDefinationId",
        to = "super::job_definition::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    JobDefination,

    #[sea_orm(has_many = "super::step_logs::Entity")]
    StepLogs,
}

impl Related<super::job_definition::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JobDefination.def()
    }
}

impl Related<super::step_logs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::StepLogs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
