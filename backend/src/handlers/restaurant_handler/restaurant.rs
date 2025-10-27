use axum::{
    extract::{State, ConnectInfo, Json},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,

};
use axum_extra::extract::cookie::CookieJar;
use std::net::SocketAddr;
use axum::extract::{Path};
use uuid::Uuid;
use crate::models::restaurants_models::restaurants::{Restaurant, RestaurantDetails};
use crate::services::restaurant_services::restaurant::{create_restaurant_service, get_restaurant_by_session_service, get_restaurant_by_supabase_service, get_restaurant_service};
use crate::state::AppState;

pub async fn create_restaurant_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Json(restaurant_details): Json<RestaurantDetails>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    
    let pool = state.pool;
    
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let ip_address = addr.ip().to_string();

    let (restaurant, cookie) = create_restaurant_service(pool.clone(), restaurant_details, Some(ip_address), Some(user_agent)).await?;

    let cookie = jar.add(cookie);

    Ok((cookie, Json(restaurant)))
}

pub async fn get_restaurant_handler(
    State(state): State<AppState>,
    Path(restaurant_name): Path<String>
)->Result<Json<Vec<Restaurant>>, (StatusCode, String)>{
    let pool = state.pool;
    
    let restaurant = get_restaurant_service(&pool, &restaurant_name)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(restaurant))
}

pub async fn get_restaurant_by_session_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<RestaurantDetails>, (StatusCode, String)> {
    let pool = state.pool;

    // 1️⃣ Try Supabase UID first
    if let Some(uid_cookie) = jar.get("supabase_uid") {
        let uid_str = uid_cookie.value();

        let supabase_uid = Uuid::parse_str(uid_str)
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid Supabase UID format".into()))?;

        let restaurant = get_restaurant_by_supabase_service(&pool, supabase_uid)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        if let Some(restaurant) = restaurant {
            return Ok(Json(restaurant));
        } else {
            return Err((StatusCode::NOT_FOUND, "Restaurant not found for this Supabase user".into()));
        }
    }

    // 2️⃣ Fallback to anonymous session
    if let Some(cookie) = jar.get("session_token") {
        let token_str = cookie.value();

        let session_token = Uuid::parse_str(token_str)
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid session token format".into()))?;

        let restaurant = get_restaurant_by_session_service(&pool, session_token)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        if let Some(restaurant) = restaurant {
            Ok(Json(restaurant))
        } else {
            Err((StatusCode::NOT_FOUND, "Restaurant not found for this session".into()))
        }
    } else {
        // 3️⃣ No valid cookie found
        Err((StatusCode::UNAUTHORIZED, "No Supabase UID or session cookie found".into()))
    }
}

