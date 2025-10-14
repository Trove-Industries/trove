use axum::{
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use http::StatusCode;
use sqlx::PgPool;
use tracing::error;

use crate::db::{
    restaurants_queries::restaurants::create_restaurant_query,
    session_query::create_session_query::create_new_session,
    user::user_queries::create_init_user,
};
use crate::models::{
    restaurants_models::restaurants::{NewRestaurant, RestaurantDetails},
    session_models::session::{NewSession},
    user_models::user::InitUser,
};

/// Creates a user, restaurant, and session — and returns a response with cookie attached
pub async fn create_session(
    ip: Option<String>,
    user_agent: Option<String>,
    pool: &PgPool,
    restaurant_details: RestaurantDetails,
) -> Result<(CookieJar, Response), (StatusCode, String)> {
    // STEP 1: Generate username based on restaurant name
    fn generate_username(restaurant_name: &str) -> String {
        restaurant_name
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .to_lowercase()
            + "_inituser"
    }

    let username = generate_username(&restaurant_details.restaurant_name);

    // STEP 2: Create initial user
    let init_user = create_init_user(pool, InitUser { username: username.clone() })
        .await
        .map_err(|e| {
            error!("❌ Failed to create user: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user".into())
        })?;

    // STEP 3: Create restaurant entry
    create_restaurant_query(
        pool,
        NewRestaurant {
            user_id: init_user.id,
            restaurant_name: restaurant_details.restaurant_name.clone(),
            restaurant_country: restaurant_details.restaurant_country.clone(),
            restaurant_city: restaurant_details.restaurant_city.clone(),
            restaurant_subdomain: restaurant_details.restaurant_subdomain.clone(),
        },
    )
        .await
        .map_err(|e| {
            error!("❌ Failed to create restaurant: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create restaurant".into())
        })?;

    // STEP 4: Create session record
    let session = create_new_session(
        pool,
        NewSession {
            user_id: init_user.id,
            ip_address: ip.clone(),
            user_agent: user_agent.clone(),
        },
    )
        .await
        .map_err(|e| {
            error!("❌ Failed to create session: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create session".into())
        })?;

    // STEP 5: Build secure session cookie
    let cookie = Cookie::build(("session_token", session.session_token.clone()))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::hours(6))
        .build();

    let jar = CookieJar::new().add(cookie);

    // STEP 6: Return success JSON with cookie
    Ok((
        jar,
        (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "message": "Session created successfully",
                "session_token": session.session_token
            })),
        )
            .into_response(),
    ))
}
