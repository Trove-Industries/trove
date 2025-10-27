use axum::Json;
use axum_extra::extract::cookie::CookieJar;
use sqlx::PgPool;
use http::StatusCode;
use serde_json::json;
use uuid::Uuid;
use crate::db::restaurants_queries::restaurants::{
    get_restaurant_id_by_session_query,
    get_restaurant_id_by_supabase_uid_query,
};


pub async fn get_restaurant_id(
    pool: &PgPool,
    jar: &CookieJar,
) -> Result<i32, (StatusCode, Json<serde_json::Value>)> {
    // 1️⃣ Check for Supabase authentication first
    if let Some(uid_cookie) = jar.get("supabase_uid") {
        let supabase_uid = uid_cookie.value();

        // Ensure valid UUID
        let parsed_uid = Uuid::parse_str(supabase_uid).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid Supabase UID format" })),
            )
        })?;

        let result = get_restaurant_id_by_supabase_uid_query(pool, &parsed_uid)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                )
            })?;

        return match result {
            Some(r) => Ok(r.id),
            None => Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Restaurant not found for this Supabase user" })),
            )),
        };
    }

    // 2️⃣ Fallback to anonymous session
    if let Some(session_cookie) = jar.get("session_token") {
        let session_token = session_cookie.value();

        // Ensure valid UUID format for session token
        let parsed_session = Uuid::parse_str(session_token).map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid session token format" })),
            )
        })?;

        let result = get_restaurant_id_by_session_query(pool, &parsed_session)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                )
            })?;

        return match result {
            Some(r) => Ok(r.id),
            None => Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Restaurant not found for this session" })),
            )),
        };
    }

    // 3️⃣ Neither cookie found → unauthorized
    Err((
        StatusCode::UNAUTHORIZED,
        Json(json!({ "error": "No Supabase or session cookie found" })),
    ))
}

// placeholder functions
pub async fn get_category_id() {}
pub async fn get_meal_group_id() {}
pub async fn get_meal_id() {}
