use sqlx::{Error, PgPool};
use crate::db::restaurants_queries::restaurants::{create_restaurant_query, get_restaurant_query};
use crate::models::restaurants_models::restaurants::{NewRestaurant, Restaurant};

pub async fn create_restaurant_service(
    pool: &PgPool,
    new_restaurant: NewRestaurant,
)->Result<Restaurant, Error>{
    create_restaurant_query(pool, new_restaurant).await
}

pub async fn get_restaurant_service(
    pool: &PgPool,
    new_restaurant: &String,
) -> Result<Vec<Restaurant>, Error> {
    get_restaurant_query(pool, new_restaurant).await
}

