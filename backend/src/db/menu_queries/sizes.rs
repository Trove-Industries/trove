use sqlx::{Error, Execute, PgPool};
use crate::models::menu_models::sizes::{NewSize, Size};

pub async fn create_size_query(
    pool: &PgPool,
    new_size: NewSize
)->Result<Size, Error>{
    let new_size = sqlx::query_as::<_,Size>(
        r#"
                INSERT INTO sizes(meal_id, restaurant_id, size_name, size_price)
                VALUES (
                        (
                            SELECT m.id
                            FROM meals m
                            JOIN meal_groups mg ON m.meal_group_id = mg.id
                            WHERE meal_group_name = $2 AND meal_name = $3
                        ),
                        (
                            SELECT r.id FROM restaurants r WHERE r.restaurant_name ILIKE $1
                        ),
                        $4, $5
                )
                RETURNING id, meal_id, restaurant_id, size_name, size_price
            "#
    )
        .bind(new_size.restaurant_name)
        .bind(new_size.meal_group_name)
        .bind(new_size.meal_name)
        .bind(new_size.size_name)
        .bind(new_size.size_price)
        .persistent(false)
        .fetch_one(pool)
        .await?;
    Ok(new_size)
}

pub async fn get_size_query(
    pool: &PgPool,
    restaurant_name: &String
)->Result<Vec<Size>, Error>{
    let size = sqlx::query_as::<_,Size>(
        r#"
                SELECT s.id, s.meal_id, s.restaurant_id, s.size_name, s.size_price
                FROM sizes s
                JOIN restaurants r ON s.restaurant_id = r.id
                WHERE r.restaurant_name ILIKE $1
            "#
    )
        .bind(restaurant_name)
        .fetch_all(pool)
        .await?;
    Ok(size)
}
