use sea_orm_migration::{MigrationTrait, MigratorTrait, async_trait};

pub mod identifiers;
pub mod migrations;

use migrations::m0001_create_job_defination;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m0001_create_job_defination::Migration)]
    }
}
