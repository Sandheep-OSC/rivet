use actix_web::{HttpResponse, Responder, get, post, web};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

use crate::{
    entities::job_instance::ActiveModel,
    identifiers::{job_status_db::JobStatusDb, job_trigger_db::JobTriggerDb},
    repository::job_instance_repo::JobInstanceRepository,
    setup::app_state::AppState,
};

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json("ok")
}

#[post("/test")]
pub async fn test_job_instance(state: web::Data<AppState>) -> impl Responder {
    let job_instance_repo = JobInstanceRepository::new(&state.db);
    let now = Utc::now().into();

    let active = ActiveModel {
        id: Set(Uuid::new_v4()),
        job_defination_id: Set("123e4567-e89b-12d3-a456-426614174000"
            .parse()
            .expect("valid UUID")),
        commit_sha: Set(Some("".to_string())),
        status: Set(JobStatusDb::Running),
        triggered_by: Set(JobTriggerDb::MANUAL),
        created_at: Set(now),
        started_at: Set(now),
        finished_at: Set(Some(now)),
    };

    match job_instance_repo.create_job_instance(active).await {
        Ok(model) => HttpResponse::Ok().json(model),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/mark-started")]
async fn mark_job_as_started(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json("sfvnjd")
}

#[post("/update-job-status")]
async fn update_job_instance_status(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json("vfv")
}

#[post("/mark-finished")]
async fn mark_job_as_finished() -> impl Responder {
    HttpResponse::Ok().json("fvn")
}

#[get("get_by_id")]
async fn get_instance_by_id() -> impl Responder {
    HttpResponse::Ok().json("lvmjf n")
}
