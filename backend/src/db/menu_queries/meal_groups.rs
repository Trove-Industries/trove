use sqlx::{Error, PgPool};
use crate::models::menu_models::meal_groups::{MealGroup, NewMealGroup};

pub async fn create_meal_group_query(
    pool: &PgPool,
    category_id: i32,
    restaurant_id: i32,
    new_meal_group: NewMealGroup,
)->Result<MealGroup, Error>{
    let new_meal_group = sqlx::query_as::<_,MealGroup>(
        r#"
                INSERT INTO meal_groups (category_id, restaurant_id, meal_group_name)
                VALUES ($1, $2, $3)
                RETURNING id, restaurant_id, category_id, meal_group_name
            "#
    )
        .bind(category_id)
        .bind(restaurant_id)
        .bind(new_meal_group.meal_group_name)
        .persistent(false)
        .fetch_one(pool)
        .await?;
    Ok(new_meal_group)
}

// browser
pub async fn get_meal_group_by_subdomain_query(
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

//app
pub async fn get_meal_group_by_session_query(
    pool: &PgPool,
    restaurant_id: i32,
)->Result<Vec<MealGroup>, Error>{
    let meal_group = sqlx::query_as::<_,MealGroup>(
        r#"
                SELECT id, category_id, restaurant_id, meal_group_name
                FROM meal_groups
                WHERE restaurant_id = $1
            "#
    )
        .bind(restaurant_id)
        .persistent(false)
        .fetch_all(pool)
        .await?;
    Ok(meal_group)
}
