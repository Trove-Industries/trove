use axum::Error;
use sqlx::{Execute, PgPool};
use crate::models::template_models::menu_template_models::{MenuItems, Restaurant};

pub async fn get_restaurant_data(
    pool: &PgPool,
    restaurant_name: &String
) -> Result<Restaurant, sqlx::Error> {
    let restaurant = sqlx::query_as::<_, Restaurant>(
        r#"
            SELECT id, restaurant_name, country, city
            FROM restaurants
            WHERE subdomain = $1
        "#
    )
        .persistent(false)
        .bind(restaurant_name)
        .fetch_one(pool)
        .await?;

    Ok(restaurant)
}


pub async fn get_menu_data(
    pool: &PgPool,
    restaurant_id: i32
) -> Result<Vec<MenuItems>, sqlx::Error>{

    let menu_data = sqlx::query_as::<_,MenuItems>(
        r#"
                SELECT *
                FROM menu_items
                WHERE restaurant_id = $1
            "#
    )
        .bind(restaurant_id)
        .persistent(false)
        .fetch_all(pool)
        .await?;

    Ok(menu_data)
}

