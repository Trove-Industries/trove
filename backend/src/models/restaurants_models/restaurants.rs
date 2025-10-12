use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Restaurant {
    pub id: i32,
    pub restaurant_name: String,
    pub restaurant_country: String,
    pub restaurant_city: String,
    pub restaurant_subdomain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRestaurant {
    pub restaurant_name: String,
    pub restaurant_country: String,
    pub restaurant_city: String,
    pub restaurant_subdomain: String,
    //pub default_currency: Option<i32>,
}
