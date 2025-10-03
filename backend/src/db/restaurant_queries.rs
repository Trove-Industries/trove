use sqlx::{Error, PgPool};
use crate::models::restaurant_models::RestaurantName;

pub async fn validate_restaurant(
    pool: &PgPool,
    restaurant_name: String,
) ->Result<Vec<RestaurantName>, Error> {
    let name = sqlx::query_as::<_, RestaurantName>(
        r#"
                SELECT DISTINCT restaurant_name
                FROM menu_items
                WHERE restaurant_name ILIKE $1
            "#
    )
        .bind(restaurant_name)
        .persistent(false)
        .fetch_all(pool)
        .await?;

    Ok(name)
}