use anyhow::{Context};
use axum::http::{HeaderValue, Method};
use crate::db::connection::connection_pool;
use crate::config::config::load_config;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing_subscriber::fmt::format;

mod config;
mod db;
mod routes;
mod services;
mod handlers;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    tracing_subscriber::fmt::init();

    let cfg = load_config()?;

    let pool = connection_pool(&cfg.database_url)
        .await
        .context("Failed to connect")?;
    println!("Connection Successful");

    let allowed_origin = HeaderValue::from_str(&cfg.allowed_origin)
        .context("ALLOWED ORIGIN format incorrect")?;

    let cors = CorsLayer::new()
        .allow_origin(allowed_origin)
        .allow_methods([Method::GET,Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = routes::menu_router::menu_routes()
        .with_state(pool)
        .layer(cors);

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .context("Port not set correctly")?;
    
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("Server running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
