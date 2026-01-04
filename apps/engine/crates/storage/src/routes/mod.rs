use actix_web::web;

mod health;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health);
    cfg.service(health::test_job_instance);
}
