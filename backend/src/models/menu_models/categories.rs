use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct Category{
    pub id: i32,
    pub restaurant_id: i32,
    pub category_name: String,
    pub category_icon: String
}

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct NewCategory {
    pub restaurant_name: String,
    pub category_name: String,
    pub category_icon: String
}