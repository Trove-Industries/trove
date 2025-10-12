use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct Pairing{
    pub id: i32,
    pub meal_id: i32,
    pub restaurant_id: i32,
    pub pairing_name: String,
    pub pairing_image: String,
    pub pairing_price: i32,
}

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct NewPairing{
    pub restaurant_name: String,
    pub meal_name: String,
    pub pairing_name: String,
    pub pairing_image: String,
    pub pairing_price: i32,
}