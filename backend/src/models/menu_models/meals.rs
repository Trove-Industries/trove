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
    pub meal_image: Option<String>
}

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct NewMeal{
    pub meal_group_id: i32,
    pub meal_name: String,
    pub meal_description: String,
    pub meal_image: Option<String>
}