use sqlx::{Error, PgPool};
use crate::models::menu_models::meals::{Meal, NewMeal};

pub async fn create_meal_query(
    pool: &PgPool,
    new_meal: NewMeal,
) ->Result<Meal, Error>{
    let new_meal = sqlx::query_as::<_,Meal>(
        r#"
                INSERT INTO meals(meal_group_id, restaurant_id, meal_name, meal_description, meal_image)
                VALUES (
                        (SELECT mg.id
                        FROM meal_groups mg
                        JOIN categories c ON mg.category_id = c.id
                        WHERE c.category_name ILIKE $2 AND mg.meal_group_name ILIKE $3),
                        (SELECT r.id FROM restaurants r WHERE r.restaurant_name ILIKE $1),
                        $4, $5, $6
                )
                RETURNING id, meal_group_id, restaurant_id, meal_name, meal_description, meal_likes, meal_image
            "#
    )
        .bind(new_meal.restaurant_name)
        .bind(new_meal.category_name)
        .bind(new_meal.meal_group_name)
        .bind(new_meal.meal_name)
        .bind(new_meal.meal_description)
        .bind(new_meal.meal_image)
        .persistent(false)
        .fetch_one(pool)
        .await?;
    Ok(new_meal)
}

pub async fn get_meal_query(
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