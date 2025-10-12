use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Currency {
    pub id: i32,
    pub currency_name: String,
    pub currency_iso: String,
    pub currency_symbol: String,
    pub currency_rate: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct NewCurrency {
    pub currency_name: String,
    pub currency_iso: String,
    pub currency_symbol: String,
    pub currency_rate: f64,
}
