mod config;
mod db;
mod error;
mod handlers;
mod models;
mod pow;
mod templates;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::Config, handlers::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::new()?;
    
    let pool = SqlitePoolOptions::new()
        .max_connections(20)
        .connect(&config.database_url)
        .await?;
    
    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/", get(home::index))
        .route("/boards", get(boards::list))
        .route("/boards/:slug", get(boards::show))
        .route("/threads/new/:board_id", get(threads::new_form).post(threads::create_begin))
        .route("/threads/:id", get(threads::show))
        .route("/api/pow/params", get(api::pow_params))
        .route("/api/pow/thread/begin", post(api::thread_begin))
        .route("/api/pow/thread/commit", post(api::thread_commit))
        .route("/api/pow/reply/begin", post(api::reply_begin))
        .route("/api/pow/reply/commit", post(api::reply_commit))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::info!("Starting server on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}