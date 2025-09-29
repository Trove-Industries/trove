use serde::{Deserialize, Serialize};

#[derive(Serialize,Debug,Deserialize)]
pub struct MenuItem {
    pub id: i32,
    pub restaurant_id: i32,
    pub food: String,
    pub description: String,
    pub price: i32,
    pub image: Option<String>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct NewMenuItem{
    pub restaurant_id: i32,
    pub food: String,
    pub description: String,
    pub price: i32,
    pub image: Option<String>,
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UpdateMenuItem{
    pub food: String,
    pub description: String,
    pub price: i32,
    pub image: Option<String>,
}

