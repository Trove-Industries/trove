use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewRestaurant{
    pub restaurant_name: String,
    pub country: String,
    pub city: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Restaurant{
    pub id: i32,
    pub restaurant_name: String,
    pub country: String,
    pub city: String,
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RestaurantName{
    pub restaurant_name: String,
}