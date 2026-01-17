use std::sync::Arc;

use control_plane::routes;
use control_plane::setup::{app, config, openapi, telemetry};

#[actix_web::main]
async fn main() -> Result<(), anyhow::Error> {
    telemetry::init_from_env();

    tracing::info!("Starting Control Plane...");

    let config = config::Config::from_env().expect("Failed to load config");
    tracing::info!("Configuration loaded successfully");

    app::register_routes();
    app::print_routes();

    let db = match connect_to_database(&config).await {
        Ok(db) => {
            tracing::info!("Database connected successfully");
            Some(db)
        }
        Err(e) => {
            tracing::warn!("Database connection failed: {}. Running without DB.", e);
            None
        }
    };

    let state = app::AppState::new(Arc::new(config), db);

    let addr = state.config.app.addr();

    tracing::info!("Server starting on {}", addr);

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .service(
                actix_web::web::resource("/api-docs/openapi.json")
                    .route(actix_web::web::get().to(openapi::openapi_json)),
            )
            .service(actix_web::web::scope("/api").configure(routes::init))
            .app_data(actix_web::web::Data::new(state.clone()))
    })
    .bind(&addr)?;

    server.run().await?;

    Ok(())
}

async fn connect_to_database(
    config: &config::Config,
) -> Result<storage::DbConnection, anyhow::Error> {
    let db_url = config.database_url();

    let db = storage::database::connect(db_url).await?;

    Ok(db)
}
