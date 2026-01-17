use actix_web::{HttpResponse, web};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
}

#[utoipa::path(
    get,
    path = "/me",
    tag = "Users",
    responses(
        (status = 200, description = "Current user info", body = UserResponse),
        (status = 401, description = "Unauthorized")
    )
)]
#[actix_web::get("/me")]
pub async fn get_me(
    state: web::Data<crate::setup::app::AppState>,
    // TODO: Add auth middleware to extract user_id
) -> Result<HttpResponse, crate::setup::error::Error> {
    let db = state
        .db
        .as_ref()
        .ok_or_else(|| crate::setup::error::Error::Database("DB not available".to_string()))?;
    // Placeholder: assume user_id from auth
    let user_id = uuid::Uuid::new_v4(); // Placeholder
    let user = crate::services::users::get_user_by_id(user_id, &**db).await?;
    Ok(HttpResponse::Ok().json(UserResponse {
        id: user.id.to_string(),
        email: user.email,
        name: user.name,
    }))
}

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(get_me);
}
