use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct MealGroup{
    pub id: i32,
    pub category_id: i32,
    pub restaurant_id: i32,
    pub meal_group_name: String
}

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct NewMealGroup{
    pub category_name: String,
    pub restaurant_name: String,
    pub meal_group_name: String
}