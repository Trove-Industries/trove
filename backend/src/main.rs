use std::net::SocketAddr;
use anyhow::Context;
use axum::http::{HeaderValue, Method};
use axum::http;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::db::connection::connection_pool;
use crate::config::config::load_config;
// use crate::middleware::session_layer::SessionLayer;
use crate::routes::main_router::main_router;
use crate::state::AppState;
use crate::utils::supabase_client::SupabaseClient;

mod config;
mod db;
mod routes;
mod services;
mod handlers;
mod models;
mod utils;
mod admin;
mod middleware;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cfg = load_config()?;
    let pool = connection_pool(&cfg.database_url)
        .await
        .context(" Failed to connect to database")?;
    println!("✅ Database connection successful");

    // start_session_cleanup_task(pool.clone()).await;

    let supabase = SupabaseClient::new();

    let app_state = AppState{
        pool,
        supabase
    };

    // --- CORS setup ---
    let origins: Vec<HeaderValue> = cfg
        .allowed_origin
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<HeaderValue>()
                .with_context(|| format!("Invalid CORS origin: {}", s))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([http::header::CONTENT_TYPE])
        .allow_origin(AllowOrigin::list(origins))
        .allow_credentials(true);

    tracing::info!("🌍 Allowed origins: {:?}", cfg.allowed_origin);


    let app = main_router()
        .with_state(app_state.clone())
        // Cookies first
        .layer(CookieManagerLayer::new())
        // Middleware for authenticated routes
        //.layer(SessionLayer::new(pool.clone(), true))
        // CORS
        .layer(cors);

    // --- Server setup ---
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .context(" Invalid or missing PORT variable")?;

    let addr: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .context(" Failed to parse server address")?;

    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Server running at http://{}", addr);

    // ✅ Allow handlers to use ConnectInfo<SocketAddr>
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .context(" Server failed to start")?;

    Ok(())
}
