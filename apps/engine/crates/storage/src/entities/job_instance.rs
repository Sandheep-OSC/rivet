use sea_orm::prelude::*;

use crate::identifiers::{job_status_db::JobStatusDb, job_trigger_db::JobTriggerDb};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "job_instance")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub job_defination_id: Uuid,
    pub commit_sha: Option<String>,
    pub status: JobStatusDb,
    pub triggered_by: JobTriggerDb,
    pub created_at: DateTimeWithTimeZone,
    pub started_at: DateTimeWithTimeZone,
    pub finished_at: Option<DateTimeWithTimeZone>,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::job_definition::Entity",
        from = "Column::JobDefinationId",
        to = "super::job_definition::Column::Id",
        on_delete = "Cascade",
        on_update = "Cascade"
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
