use actix_web::HttpResponse;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::health::health,
        crate::routes::auth::login,
        crate::routes::auth::register,
        crate::routes::users::get_me,
    ),
    components(schemas(
        crate::routes::health::HealthResponse,
        crate::routes::auth::LoginRequest,
        crate::routes::auth::LoginResponse,
        crate::routes::auth::RegisterRequest,
        crate::routes::auth::RegisterResponse,
        crate::routes::users::UserResponse,
    )),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Users", description = "User management endpoints"),
    )
)]
struct ApiDoc;

pub async fn openapi_json() -> HttpResponse {
    HttpResponse::Ok().json(ApiDoc::openapi())
}
