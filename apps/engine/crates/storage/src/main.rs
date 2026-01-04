use dotenvy::dotenv;
use storage::setup::{app::run_server, config::Config, db::init_db, logging::init_logging};
use tracing::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::from_env();
    init_logging();

    info!("Starting application");

    let db = init_db(&config.database_url).await;

    run_server(config.server_port, db).await
}
