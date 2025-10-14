use sqlx::{Error, PgPool};
use crate::models::restaurants_models::restaurants::{NewRestaurant, Restaurant};

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
