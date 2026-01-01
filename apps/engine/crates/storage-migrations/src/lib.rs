use sea_orm_migration::{MigrationTrait, MigratorTrait, async_trait};

pub mod identifiers;
pub mod migrations;

use migrations::{
    m0001_create_job_defination, m0002_create_job_step, m0003_create_job_trigger_enum,
    m0004_create_job_status_enum,
};

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m0001_create_job_defination::Migration),
            Box::new(m0002_create_job_step::Migration),
            Box::new(m0003_create_job_trigger_enum::Migration),
            Box::new(m0004_create_job_status_enum::Migration),
        ]
    }
}
