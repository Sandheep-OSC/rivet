use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};

use crate::entities::job_instance::*;

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
}
