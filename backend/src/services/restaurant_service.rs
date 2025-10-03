use sqlx::PgPool;
use crate::db::restaurant_queries;
use crate::models::restaurant_models::RestaurantName;

pub async fn validate_restaurant(
    pool: &PgPool,
    restaurant_name: String
) ->Result<Vec<RestaurantName>, sqlx::Error> {
    
    restaurant_queries::validate_restaurant(pool, restaurant_name).await
}