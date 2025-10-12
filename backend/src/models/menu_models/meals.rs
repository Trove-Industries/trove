use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug,Deserialize,Serialize,FromRow)]

pub struct Meal{
    pub id: i32,
    pub meal_group_id: i32,
    pub restaurant_id: i32,
    pub meal_name: String,
    pub meal_description: String,
    pub meal_likes: Option<i32>,
    pub meal_image: String
}

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct NewMeal{
    pub restaurant_name: String,
    pub category_name: String,
    pub meal_group_name: String,
    pub meal_name: String,
    pub meal_description: String,
    pub meal_image: String
}