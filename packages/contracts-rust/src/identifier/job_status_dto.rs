use engine_core::types::job_status::JobStatus;
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
pub enum JobStatusDTO {
    Pending,
    Running,
    Success,
    Failed,
}

impl From<JobStatus> for JobStatusDTO {
    fn from(status_dto: JobStatus) -> Self {
        match status_dto {
            JobStatus::Running => Self::Running,
            JobStatus::Pending => Self::Pending,
            JobStatus::Success => Self::Success,
            JobStatus::Failed => Self::Failed,
        }
    }
}
