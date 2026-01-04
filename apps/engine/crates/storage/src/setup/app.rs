use actix_web::{App, HttpServer, middleware::Logger, web};
use sea_orm::DatabaseConnection;

use crate::{routes, setup::app_state::AppState};

pub async fn run_server(port: u16, db: DatabaseConnection) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .configure(routes::init)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
