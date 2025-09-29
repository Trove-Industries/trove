use sqlx::{Error, PgPool};
use crate::models::menu_item::{MenuItem, NewMenuItem};

pub async fn insert_menu(
    pool: &PgPool,
    new_item: NewMenuItem
) -> Result<MenuItem, Error> {
    let item = sqlx::query_as!(
        MenuItem,
        r#"
            INSERT INTO menu_items (restaurant_id, food, description, price, image)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, restaurant_id, food, description, price, image
        "#,
        new_item.restaurant_id,
        new_item.food,
        new_item.description,
        new_item.price,
        new_item.image,
    )
        .fetch_one(pool)
        .await?;

    Ok(item)
}

pub async fn get_menu_by_restaurant(
    pool: &PgPool,
    restaurant_id: i32,
) ->Result<Vec<MenuItem>, Error>{
    let menu = sqlx::query_as!(
        MenuItem,
        r#"
            SELECT id, restaurant_id, food, description, price, image
            FROM menu_items
            WHERE restaurant_id = $1
        "#,
        restaurant_id
    )
        .fetch_all(pool)
        .await?;

    Ok(menu)

}

