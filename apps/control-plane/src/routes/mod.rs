pub mod auth;
pub mod health;
pub mod users;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(actix_web::web::scope("/auth").configure(auth::init))
        .service(actix_web::web::scope("/users").configure(users::init))
        .service(actix_web::web::scope("/health").configure(health::init));
}
