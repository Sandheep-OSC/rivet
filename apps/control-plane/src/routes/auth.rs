use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,
}

#[utoipa::path(
    post,
    path = "/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
#[actix_web::post("/login")]
pub async fn login(
    req: web::Json<LoginRequest>,
    state: web::Data<crate::setup::app::AppState>,
) -> Result<HttpResponse, crate::setup::error::Error> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| crate::setup::error::Error::Database("DB not available".to_string()))?;
    let token = crate::services::auth::login(&req.email, &req.password, db).await?;
    Ok(HttpResponse::Ok().json(LoginResponse { token }))
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct RegisterResponse {
    pub message: String,
}

#[utoipa::path(
    post,
    path = "/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered", body = RegisterResponse),
        (status = 400, description = "Validation error")
    )
)]
#[actix_web::post("/register")]
pub async fn register(
    req: web::Json<RegisterRequest>,
    state: web::Data<crate::setup::app::AppState>,
) -> Result<HttpResponse, crate::setup::error::Error> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| crate::setup::error::Error::Database("DB not available".to_string()))?;
    crate::services::auth::register(&req.email, &req.password, &req.name, db).await?;
    Ok(HttpResponse::Created().json(RegisterResponse {
        message: "User registered successfully".to_string(),
    }))
}

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(login).service(register);
}
