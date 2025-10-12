use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Debug, Deserialize, FromRow)]
pub struct MenuItem {
    pub id: i32,
    pub food: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewMenuItem {
    pub restaurant_name: String,
    pub food: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UpdateMenuItem {
    pub food: String,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub image: Option<String>,
}

