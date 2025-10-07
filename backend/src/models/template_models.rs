use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct Restaurant{
    pub id: i32,
    pub restaurant_name: String,
    pub country: String,
    pub city: String,
}

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct MenuItems{
    pub id: i32,
    pub restaurant_id: i32,
    pub food: String,
    pub description: Option<String>,
    pub price: i32,
    pub image: Option<String>,
}

#[derive(Debug,Serialize,Deserialize,FromRow)]
pub struct FullData {
    pub menu_items: Vec<MenuItems>,
    pub restaurant: Restaurant
}