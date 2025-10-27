use axum_extra::extract::cookie::{Cookie, SameSite};
use http::StatusCode;
use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::db::restaurants_queries::restaurants::{create_restaurant_query, get_restaurant_by_session_query, get_restaurant_by_supabase_query, get_restaurant_query};
use crate::db::user::user_queries::create_init_user;
use crate::models::restaurants_models::restaurants::{NewRestaurant, Restaurant, RestaurantDetails};
use crate::models::user_models::user::InitUser;
use crate::services::session_services::session::create_session_service;
use crate::utils::generate_username::generate_username;

pub async fn create_restaurant_service(
    pool: PgPool,
    restaurant_details: RestaurantDetails,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<(Restaurant, Cookie<'static>), (StatusCode, String)> {

    // Step 1: Create init user
    let username = generate_username(&restaurant_details.restaurant_name);
    let init_user = create_init_user(&pool, InitUser { username: username.clone() })
        .await
        .map_err(|e|{
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create user {}", e)
                )
        })?;

    let session = create_session_service(&pool, init_user.id, ip_address, user_agent)
        .await
        .map_err(|_|(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create Session".into()))?;

    let mut cookie = Cookie::new("session_token", session.session_token.to_string());
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Lax);
    cookie.set_path("/");
    // TODO set to true in deployment
    cookie.set_secure(false);


    // Step 2: Create restaurant
    let restaurant = create_restaurant_query(&pool, NewRestaurant {
        user_id: init_user.id,
        restaurant_name: restaurant_details.restaurant_name.clone(),
        restaurant_country: restaurant_details.restaurant_country.clone(),
        restaurant_city: restaurant_details.restaurant_city.clone(),
        restaurant_subdomain: restaurant_details.restaurant_subdomain.clone(),
    })
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create restaurant".into()))?;



    Ok((restaurant, cookie))
}

pub async fn get_restaurant_service(
    pool: &PgPool,
    new_restaurant: &String,
) -> Result<Vec<Restaurant>, Error> {
    get_restaurant_query(pool, new_restaurant).await
}


pub async fn get_restaurant_by_session_service(
    pool: &PgPool,
    session_token: Uuid,
) -> Result<Option<RestaurantDetails>, sqlx::Error> {
    let result = get_restaurant_by_session_query(pool, session_token).await?;
    Ok(result.into_iter().next())
}

pub async fn get_restaurant_by_supabase_service(
    pool: &PgPool,
    supabase_uid: Uuid,
) -> Result<Option<RestaurantDetails>, sqlx::Error> {
    let result = get_restaurant_by_supabase_query(pool, supabase_uid).await?;
    Ok(result.into_iter().next())
}


