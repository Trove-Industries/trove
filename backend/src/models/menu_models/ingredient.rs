use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct Ingredient{
    pub id: i32,
    pub meal_id: i32,
    pub restaurant_id: i32,
    pub ingredient_name: String,
    pub ingredient_image: Option<String>,
}

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct NewIngredient{
    pub restaurant_name: String,
    pub meal_name: String,
    pub ingredient_name: String,
    pub ingredient_image: Option<String>,
}