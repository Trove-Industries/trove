use sqlx::{Error, PgPool};
use crate::models::restaurant_models::{NewRestaurant, Restaurant, RestaurantName};


pub async fn register_restaurant(
    pool: &PgPool,
    new_restaurant: NewRestaurant,
) ->Result<Restaurant, Error> {

    let restaurant = sqlx::query_as::<_,Restaurant>(
        r#"
                INSERT INTO restaurants (restaurant_name, country, city, subdomain)
                VALUES ($1, $2, $3, $4)
                RETURNING id, restaurant_name, country, city
            "#
    )

        .bind(new_restaurant.restaurant_name)
        .bind(new_restaurant.country)
        .bind(new_restaurant.city)
        .bind(new_restaurant.subdomain)
        .persistent(false)
        .fetch_one(pool)
        .await?;

    Ok(restaurant)
}

pub async fn validate_restaurant(
    pool: &PgPool,
    restaurant_name: String,
) ->Result<Vec<RestaurantName>, Error> {
    let name = sqlx::query_as::<_, RestaurantName>(
        r#"
                SELECT DISTINCT restaurant_name
                FROM restaurants
                WHERE restaurant_name ILIKE $1
            "#
    )
        .bind(restaurant_name)
        .persistent(false)
        .fetch_all(pool)
        .await?;

    Ok(name)
}

