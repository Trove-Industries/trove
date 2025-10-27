use axum::{
    extract::{State},
    response::{IntoResponse},
    Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use tower_cookies::Cookies;
use http::StatusCode;
use tracing::error;
use sqlx::PgPool;


use crate::models::user_models::user::User;
use crate::services::session_services::session::create_session_service;
// adjust to your user model

pub async fn create_session_handler(
    State(pool): State<PgPool>,
    cookies: Cookies,
    Json(user): Json<User>,
) -> impl IntoResponse {
    match create_session_service(
        &pool,
        user.id,
        None, // later, we can extract real IP and User-Agent
        None,
    )
        .await
    {
        Ok(session) => {
            // 🍪 Set secure cookie
            let mut cookie = Cookie::new("session_token", session.session_token.to_string());
            cookie.set_http_only(true);
            cookie.set_same_site(SameSite::Lax);
            cookie.set_path("/");
            cookie.set_secure(true); // only over HTTPS in production
            cookies.add(cookie);

            (
                StatusCode::OK,
                Json(serde_json::json!({
                    "message": "Session created successfully",
                    "session_token": session.session_token,
                    "expires_at": session.expires_at
                })),
            )
        }
        Err(e) => {
            error!("❌ Failed to create session: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "Failed to create session"
                })),
            )
        }
    }
}
