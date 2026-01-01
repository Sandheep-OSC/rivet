use engine_core::types::job_trigger::JobTrigger;
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
pub enum JobTriggerDto {
    Webhook,
    Manual,
    Cron,
}

impl From<JobTrigger> for JobTriggerDto {
    fn from(trigger: JobTrigger) -> Self {
        match trigger {
            JobTrigger::Webhook => Self::Webhook,
            JobTrigger::Manual => Self::Manual,
            JobTrigger::Cron => Self::Cron,
        }
    }
}
