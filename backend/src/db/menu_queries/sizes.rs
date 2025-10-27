use sqlx::{Error, Execute, PgPool};
use crate::models::menu_models::sizes::{NewSize, Size};

pub async fn create_size_query(
    pool: &PgPool,
    meal_id: i32,
    restaurant_id: i32,
    new_size: NewSize
)->Result<Size, Error>{
    let new_size = sqlx::query_as::<_,Size>(
        r#"
                INSERT INTO sizes(meal_id, restaurant_id, size_name, size_price)
                VALUES ($1, $2, $3, $4)
                RETURNING id, meal_id, restaurant_id, size_name, size_price
            "#
    )
        .bind(meal_id)
        .bind(restaurant_id)
        .bind(new_size.size_name)
        .bind(new_size.size_price)
        .persistent(false)
        .fetch_one(pool)
        .await?;
    Ok(new_size)
}

pub async fn get_size_by_subdomain_query(
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
        .persistent(false)
        .fetch_all(pool)
        .await?;
    Ok(size)
}

pub async fn get_size_by_session_query(
    pool: &PgPool,
    restaurant_id: i32,
)->Result<Vec<Size>, Error>{
    let size = sqlx::query_as::<_,Size>(
        r#"
                SELECT id, meal_id, restaurant_id, size_name, size_price
                FROM sizes
                WHERE restaurant_id = $1
            "#
    )
        .bind(restaurant_id)
        .persistent(false)
        .fetch_all(pool)
        .await?;
    Ok(size)
}
