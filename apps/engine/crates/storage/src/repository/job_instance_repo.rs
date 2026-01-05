use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DbErr, EntityTrait};
use uuid::Uuid;

use crate::{entities::job_instance::*, identifiers::job_status_db::JobStatusDb};

pub struct JobInstanceRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> JobInstanceRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        JobInstanceRepository { db }
    }

    pub async fn create_job_instance(&self, instance_data: ActiveModel) -> Result<Model, DbErr> {
        let new_job_instance = ActiveModel { ..instance_data };

        new_job_instance.insert(self.db).await
    }

    pub async fn mark_job_as_started(&self, job_id: String) -> Result<Model, DbErr> {
        let job_id = Uuid::parse_str(&job_id).map_err(|_| DbErr::Custom("Invalid UUID".into()))?;
        let job = Entity::find_by_id(job_id)
            .one(self.db)
            .await? // handle DbErr
            .ok_or(DbErr::RecordNotFound("Job not found".into()))?; // handle Option

        let mut job: ActiveModel = job.into();

        job.status = Set(JobStatusDb::Running);

        job.update(self.db).await
    }
}
