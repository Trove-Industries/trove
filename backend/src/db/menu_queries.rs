use sqlx::{Error, PgPool};
use crate::models::menu_models::{MenuItem, NewMenuItem};


pub async fn insert_menu(
    pool: &PgPool,
    new_item: NewMenuItem
) -> Result<MenuItem, sqlx::Error> {
    let item = sqlx::query_as::<_, MenuItem>(
        r#"
        INSERT INTO menu_items (restaurant_id, food, description, price, image)
        VALUES (
            (SELECT id FROM restaurants WHERE restaurant_name = $1),
            $2, $3, $4, $5
        )
        RETURNING id, restaurant_id, food, description, price, image
    "#
    )
        .bind(&new_item.restaurant_name)
        .bind(&new_item.food)
        .bind(&new_item.description)
        .bind(&new_item.price)
        .bind(&new_item.image)
        .persistent(false)
        .fetch_one(pool)
        .await?;

    Ok(item)
}

pub async fn get_menu_by_restaurant(
    pool: &PgPool,
    restaurant_name: String,
) -> Result<Vec<MenuItem>, sqlx::Error> {
    let menu = sqlx::query_as::<_, MenuItem>(
        r#"
        SELECT
            m.id,
            r.restaurant_name,
            m.food,
            m.description,
            m.price,
            m.image
        FROM menu_items m
        INNER JOIN restaurants r ON m.restaurant_id = r.id
        WHERE r.restaurant_name ILIKE $1
        "#
    )
        .bind(&restaurant_name)
        .persistent(false)
        .fetch_all(pool)
        .await?;

    Ok(menu)
}


