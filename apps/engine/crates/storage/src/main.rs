mod db;

use axum::{Extension, Router, routing::get};
use dotenvy::dotenv;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env
    dotenv().ok();

    // Logging setup: explicitly setting default level to INFO
    fmt()
        .with_env_filter(EnvFilter::new("info")) // This ensures logs at INFO level and higher are shown
        .init();

    // Database setup
    let database_url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL missing")?;
    let db = db::connection::Db::init(&database_url).await?;

    // Read server port from env, default to 3000
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    // Bind TcpListener
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await?;
    let addr = listener.local_addr()?; // actual bound address

    // Build Axum router
    let app = Router::new()
        .route("/", get(|| async { "Welcome to the API!" }))
        .route("/health", get(|| async { "OK" }))
        .route("/test-transaction", get(test_transaction))
        .layer(Extension(db));

    tracing::info!("🚀 Server starting at http://{}", addr);

    // Start server
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

/// Example transaction endpoint
async fn test_transaction(
    Extension(db): Extension<db::connection::Db>,
) -> impl axum::response::IntoResponse {
    let result = db::transaction::transaction(&db.conn, |_txn| async move {
        Ok::<_, sea_orm::DbErr>("Transaction ran successfully!")
    })
    .await;

    match result {
        Ok(msg) => (axum::http::StatusCode::OK, msg.to_string()),
        Err(err) => {
            tracing::error!("Transaction failed: {:?}", err);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Transaction failed".to_string(),
            )
        }
    }
}
