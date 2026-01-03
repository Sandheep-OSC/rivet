use engine_core::types::job_trigger::JobTrigger;
use sea_orm::{DeriveActiveEnum, DeriveIden, EnumIter};

#[derive(Debug, Clone, PartialEq, Eq, DeriveActiveEnum, EnumIter, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "job_trigger")]
pub enum JobTriggerDb {
    #[sea_orm(string_value = "WEBHOOK")]
    Webhook,
    #[sea_orm(string_value = "MANUAL")]
    Manual,
    #[sea_orm(string_value = "CRON")]
    Cron,
}

impl From<JobTrigger> for JobTriggerDb {
    fn from(trigger: JobTrigger) -> Self {
        match trigger {
            JobTrigger::Webhook => Self::Webhook,
            JobTrigger::Manual => Self::Manual,
            JobTrigger::Cron => Self::Cron,
        }
    }
}
