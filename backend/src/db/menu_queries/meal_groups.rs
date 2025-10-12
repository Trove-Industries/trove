use sqlx::{Error, PgPool};
use crate::models::menu_models::meal_groups::{MealGroup, NewMealGroup};

pub async fn create_meal_group_query(
    pool: &PgPool,
    new_meal_group: NewMealGroup,
)->Result<MealGroup, Error>{
    let new_meal_group = sqlx::query_as::<_,MealGroup>(
        r#"
                INSERT INTO meal_groups (category_id, restaurant_id, meal_group_name)
                VALUES (
                        (SELECT c.id
                        FROM categories c
                        JOIN restaurants r ON c.restaurant_id = r.id
                        WHERE c.category_name ILIKE $2 AND r.restaurant_name ILIKE $1),
                        (SELECT r.id FROM restaurants r WHERE r.restaurant_name ILIKE $1),
                    $3
                )
                RETURNING id, restaurant_id, category_id, meal_group_name
            "#
    )
        .bind(new_meal_group.restaurant_name)
        .bind(new_meal_group.category_name)
        .bind(new_meal_group.meal_group_name)
        .persistent(false)
        .fetch_one(pool)
        .await?;
    Ok(new_meal_group)
}

pub async fn get_meal_group_query(
    pool: &PgPool,
    restaurant_name: &String
)->Result<Vec<MealGroup>, Error>{
    let meal_group = sqlx::query_as::<_,MealGroup>(
        r#"
                SELECT mg.id, mg.category_id, mg.restaurant_id, mg.meal_group_name
                FROM meal_groups mg
                JOIN restaurants r ON mg.restaurant_id = r.id
                WHERE r.restaurant_name ILIKE $1
            "#
    )
        .bind(restaurant_name)
        .persistent(false)
        .fetch_all(pool)
        .await?;
    Ok(meal_group)
}
