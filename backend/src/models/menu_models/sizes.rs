use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct Size{
    pub id: i32,
    pub meal_id: i32,
    pub restaurant_id: i32,
    pub size_name: String,
    pub size_price: i32
}

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct NewSize{
    pub meal_group_name: String,
    pub restaurant_name: String,
    pub meal_name: String,
    pub size_name: String,
    pub size_price: i32
}