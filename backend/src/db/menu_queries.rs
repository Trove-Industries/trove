use sqlx::{Error, PgPool, FromRow, Execute};
use crate::models::menu_item::{MenuItem, NewMenuItem, RestaurantName};

pub async fn insert_menu(
    pool: &PgPool,
    new_item: NewMenuItem
) -> Result<MenuItem, Error> {
    // Changed from query_as! to query_as
    let item = sqlx::query_as::<_, MenuItem>(
        r#"
            INSERT INTO menu_items (restaurant_name, food, description, price, image)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, restaurant_name, food, description, price, image
        "#
    )
        .bind(new_item.restaurant_name)
        .bind(new_item.food)
        .bind(new_item.description)
        .bind(new_item.price)
        .bind(new_item.image)
        .persistent(false)  // Important for pgbouncer
        .fetch_one(pool)
        .await?;

    Ok(item)
}

pub async fn get_menu_by_restaurant(
    pool: &PgPool,
    restaurant_name: String,
) -> Result<Vec<MenuItem>, Error> {
    // Changed from query_as! to query_as
    let menu = sqlx::query_as::<_, MenuItem>(
        r#"
            SELECT id, restaurant_name, food, description, price, image
            FROM menu_items
            WHERE restaurant_name ILIKE $1
        "#
    )
        .bind(restaurant_name)
        .persistent(false)  // Important for pgbouncer
        .fetch_all(pool)
        .await?;

    Ok(menu)
}

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