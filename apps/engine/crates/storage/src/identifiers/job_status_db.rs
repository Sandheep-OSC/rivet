use engine_core::types::job_status::JobStatus;
use sea_orm::{DeriveActiveEnum, DeriveIden, EnumIter};

#[derive(Debug, Clone, DeriveActiveEnum, EnumIter, DeriveIden)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "job_instance")]
pub enum JobStatusDb {
    #[sea_orm(string_value = "pending")]
    Pending,

    #[sea_orm(string_value = "running")]
    Running,

    #[sea_orm(string_value = "success")]
    Success,

    #[sea_orm(string_value = "error")]
    Error,
}

impl From<JobStatus> for JobStatusDb {
    fn from(status: JobStatus) -> Self {
        match status {
            JobStatus::Pending => Self::Pending,
            JobStatus::Success => Self::Success,
            JobStatus::Failed => Self::Error,
            JobStatus::Running => Self::Running,
        }
    }
}
