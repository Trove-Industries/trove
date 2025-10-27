use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::models::restaurants_models::restaurants::{NewRestaurant, Restaurant, RestaurantDetails, RestaurantIdBySession};

pub async fn create_restaurant_query(
    pool: &PgPool,
    restaurant_details: NewRestaurant,
) -> Result<Restaurant, Error> {
    let new_restaurant = sqlx::query_as::<_, Restaurant>(
        r#"
        INSERT INTO restaurants (user_id, restaurant_name, restaurant_country, restaurant_city, restaurant_subdomain)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, user_id, restaurant_name, restaurant_country, restaurant_city, restaurant_subdomain
        "#
    )
        .bind(restaurant_details.user_id)
        .bind(&restaurant_details.restaurant_name)
        .bind(&restaurant_details.restaurant_country)
        .bind(&restaurant_details.restaurant_city)
        .bind(&restaurant_details.restaurant_subdomain)
        .persistent(false)
        .fetch_one(pool)
        .await?;

    Ok(new_restaurant)
}


pub async fn get_restaurant_query(
    pool: &PgPool,
    restaurant_name: &String,
) -> Result<Vec<Restaurant>, Error> {
    let restaurants = sqlx::query_as::<_, Restaurant>(
        r#"
                SELECT
                    r.id,
                    r.user_id,
                    r.restaurant_name,
                    r.restaurant_country,
                    r.restaurant_city,
                    r.restaurant_subdomain
                FROM restaurants r
                WHERE r.restaurant_name ILIKE $1
               "#
    )

        .bind(format!("%{}%", restaurant_name))
        .persistent(false)
        .fetch_all(pool)
        .await?;

    Ok(restaurants)
}

pub async fn get_restaurant_by_session_query(
    pool: &PgPool,
    session: Uuid,
)->Result<Vec<RestaurantDetails>, Error>{
    let restaurant = sqlx::query_as::<_,RestaurantDetails>(
        r#"
                SELECT r.restaurant_name, r.restaurant_country, r.restaurant_city, r.restaurant_subdomain
                FROM sessions s
                JOIN users u ON s.user_id = u.id
                JOIN restaurants r ON r.user_id = u.id
                WHERE s.session_token = $1
                  AND s.is_active = TRUE
                LIMIT 1;
            "#
    )
        .bind(session)
        .persistent(false)
        .fetch_all(pool)
        .await?;
    Ok(restaurant)
}

pub async fn get_restaurant_by_supabase_query(
    pool: &PgPool,
    supabase_uid: Uuid,
) -> Result<Vec<RestaurantDetails>, sqlx::Error> {
    let restaurant = sqlx::query_as::<_,RestaurantDetails>(
        r#"
                SELECT r.*
                FROM restaurants r
                JOIN users u ON r.user_id = u.id
                WHERE u.supabase_uid = $1
            "#,
    )
        .bind(supabase_uid)
        .persistent(false)
        .fetch_all(pool)
        .await?;
    Ok(restaurant)
}


pub async fn get_restaurant_id_by_session_query(
    pool: &PgPool,
    session_token: &Uuid,
) -> Result<Option<RestaurantIdBySession>, Error> {
    let result = sqlx::query_as::<_, RestaurantIdBySession>(
        r#"
                SELECT r.id
                FROM restaurants r
                INNER JOIN users u ON r.user_id = u.id
                INNER JOIN sessions s ON s.user_id = u.id
                WHERE s.session_token = $1
        "#
    )
        .bind(session_token)
        .persistent(false)
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

pub async fn get_restaurant_id_by_supabase_uid_query(
    pool: &PgPool,
    supabase_uid: &Uuid,
)-> Result<Option<RestaurantIdBySession>, Error>{
    let result = sqlx::query_as::<_, RestaurantIdBySession>(
        r#"
                SELECT r.id
                FROM restaurants r
                INNER JOIN users u ON r.user_id = u.id
                WHERE u.supabase_uid = $1
            "#
    )
        .bind(supabase_uid)
        .persistent(false)
        .fetch_optional(pool)
        .await?;

    Ok(result)
}