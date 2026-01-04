use engine_core::types::job_trigger::JobTrigger;
use sea_orm::{DeriveActiveEnum, DeriveIden, EnumIter};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, EnumIter, DeriveIden, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "job_trigger")]
pub enum JobTriggerDb {
    #[sea_orm(string_value = "WEBHOOK")]
    WEBHOOK,
    #[sea_orm(string_value = "MANUAL")]
    MANUAL,
    #[sea_orm(string_value = "CRON")]
    CRON,
}

impl From<JobTrigger> for JobTriggerDb {
    fn from(trigger: JobTrigger) -> Self {
        match trigger {
            JobTrigger::Webhook => Self::WEBHOOK,
            JobTrigger::Manual => Self::MANUAL,
            JobTrigger::Cron => Self::CRON,
        }
    }
}
