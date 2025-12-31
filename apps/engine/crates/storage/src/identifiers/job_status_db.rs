use engine_core::types::job_status::JobStatus;

pub enum JobStatusDb {
    Pending,
    Running,
    Success,
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
