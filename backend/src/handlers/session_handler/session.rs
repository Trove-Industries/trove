use std::net::SocketAddr;
use axum::{
    extract::{ConnectInfo, State},
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::CookieJar;
use http::StatusCode;
use sqlx::PgPool;
use tracing::info;

use crate::models::restaurants_models::restaurants::RestaurantDetails;
use crate::services::user_services::user::create_session;

#[axum::debug_handler]
pub async fn start_trial_session(
    State(pool): State<PgPool>,
    jar: CookieJar,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(restaurant_details): Json<RestaurantDetails>,
) -> Result<(CookieJar, Response), (StatusCode, String)> {
    info!("🌍 Starting trial session for: {}", restaurant_details.restaurant_name);

    let ip = Some(addr.ip().to_string());

    // ✅ Extract user-agent safely from headers
    let user_agent = headers
        .get(http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    create_session(ip, user_agent, &pool, restaurant_details).await
}
