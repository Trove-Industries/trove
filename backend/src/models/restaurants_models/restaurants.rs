use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Restaurant {
    pub id: i32,
    pub user_id: i32,
    pub restaurant_name: String,
    pub restaurant_country: String,
    pub restaurant_city: String,
    pub restaurant_subdomain: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewRestaurant {
    pub user_id: i32,
    pub restaurant_name: String,
    pub restaurant_country: String,
    pub restaurant_city: String,
    pub restaurant_subdomain: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RestaurantDetails {
    //pub user_id: i32,
    pub restaurant_name: String,
    pub restaurant_country: String,
    pub restaurant_city: String,
    pub restaurant_subdomain: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RestaurantSessionQuery {
    pub session: Uuid,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RestaurantIdBySession {
    pub id: i32,
}
