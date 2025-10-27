use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use axum::{Json, response::IntoResponse, http::StatusCode};
use axum::extract::State;
use serde_json::json;
use crate::state::AppState;
use time::Duration;
use crate::models::auth_models::login::LoginRequest;
use crate::services::auth_services::login::login_service;

pub async fn login_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(login_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let pool = &state.pool;
    let supabase = &state.supabase;

    let login = login_service(pool, supabase, login_request)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, Json(json!({ "error": e }))))?;
    
    let remove_session = Cookie::build(("session_token", ""))
        .path("/")
        .max_age(Duration::seconds(0))
        .http_only(true)
        .same_site(SameSite::Lax)
        .build(); // if your cookie crate warns, swap to .build() depending on version
    
    let mut new_jar = jar.remove(remove_session);

    if let Some(user_id) = login.user_id.clone() {
        let uid_cookie = Cookie::build(("supabase_uid", user_id.clone()))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            // set secure(true) in production when using HTTPS
            //.secure(true)
            .max_age(Duration::days(7))
            .build(); // or .build() depending on cookie crate version

        new_jar = new_jar.add(uid_cookie);
    }

    // 4) set access_token cookie (http-only) if present
    if let Some(access_token) = login.access_token.clone() {
        let token_cookie = Cookie::build(("supabase_access_token", access_token.clone()))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            // .secure(true) // enable in prod
            .max_age(Duration::days(7))
            .build();

        new_jar = new_jar.add(token_cookie);
    }

    // 5) optionally set refresh token if you want to store it (be mindful of security)
    if let Some(refresh_token) = login.refresh_token.clone() {
        let refresh_cookie = Cookie::build(("supabase_refresh_token", refresh_token.clone()))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            // .secure(true)
            .max_age(Duration::days(30))
            .finish();

        new_jar = new_jar.add(refresh_cookie);
    }

    // 6) return cookies + JSON response
    Ok((new_jar, Json(login)))
}
