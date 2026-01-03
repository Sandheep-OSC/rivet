use sea_orm::prelude::*;

use crate::identifiers::job_status_db::JobStatusDb;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "step_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(indexed)]
    pub job_instance_id: Uuid,
    pub step_id: Uuid,
    #[sea_orm(indexed)]
    pub step_order: i32,
    pub status: JobStatusDb,
    pub started_at: TimeDateTimeWithTimeZone,
    pub finished_at: Option<TimeDateTimeWithTimeZone>,
    pub created_at: TimeDateTimeWithTimeZone,
}

#[derive(Clone, Debug, PartialEq, EnumIter, Eq, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::job_instance::Entity",
        from = "Column::JobInstanceId",
        to = "super::job_instance::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    JobInstance,
    #[sea_orm(
        belongs_to = "super::job_steps::Entity",
        from = "Column::StepId",
        to = "super::job_steps::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
    )]
    JobStep,
}

impl Related<super::job_instance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JobInstance.def()
    }
}

impl Related<super::job_steps::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JobStep.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
