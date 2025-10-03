use sqlx::{Error, PgPool};
use crate::db::restaurant_queries;
use crate::models::restaurant_models::{NewRestaurant, Restaurant, RestaurantName};


pub async fn register_restaurant(
    pool: &PgPool,
    new_restaurant: NewRestaurant,
) ->Result<Restaurant, sqlx::Error>{
    restaurant_queries::register_restaurant(pool,new_restaurant).await
}


pub async fn validate_restaurant(
    pool: &PgPool,
    restaurant_name: String
) ->Result<Vec<RestaurantName>, sqlx::Error> {

    restaurant_queries::validate_restaurant(pool, restaurant_name).await
}