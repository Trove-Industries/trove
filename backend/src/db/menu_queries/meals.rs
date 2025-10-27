use sqlx::{Error, PgPool};
use crate::models::menu_models::meals::{Meal, NewMeal};

pub async fn create_meal_query(
    pool: &PgPool,
    meal_group_id: i32,
    restaurant_id: i32,
    new_meal: NewMeal,
) -> Result<Meal, Error> {
    let new_meal = sqlx::query_as::<_, Meal>(
        r#"
        INSERT INTO meals(meal_group_id, restaurant_id, meal_name, meal_description, meal_likes, meal_image)
        VALUES ($1,$2, $3, $4, 0, $5)
        RETURNING id, meal_group_id, restaurant_id, meal_name, meal_description, meal_likes, meal_image
        "#
    )
        .bind(meal_group_id)
        .bind(restaurant_id)
        .bind(new_meal.meal_name)
        .bind(new_meal.meal_description)
        .bind(new_meal.meal_image)
        .persistent(false)
        .fetch_one(pool)
        .await?;
    Ok(new_meal)
}
pub async fn get_meal_by_subdomain_query(
    pool: &PgPool,
    restaurant_name: &String
)->Result<Vec<Meal>, Error>{
    let meal = sqlx::query_as::<_,Meal>(
        r#"
                SELECT m.id, m.meal_group_id, m.restaurant_id, m.meal_name, m.meal_description, m.meal_likes, m.meal_image
                FROM meals m
                JOIN restaurants r ON m.restaurant_id = r.id
                WHERE r.restaurant_name ILIKE $1
            "#
    )
        .bind(restaurant_name)
        .persistent(false)
        .fetch_all(pool)
        .await?;
    Ok(meal)
}

pub async fn get_meal_by_session_query(
    pool: &PgPool,
    restaurant_id: i32,
)->Result<Vec<Meal>, Error>{
    let meal = sqlx::query_as::<_,Meal>(
        r#"
                SELECT id, meal_group_id, restaurant_id, meal_name, meal_description, meal_likes, meal_image
                FROM meals
                WHERE restaurant_id = $1
            "#
    )
        .bind(restaurant_id)
        .persistent(false)
        .fetch_all(pool)
        .await?;
    Ok(meal)
}