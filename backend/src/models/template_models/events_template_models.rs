use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct Events{
    pub id: i32,
    pub restaurant_id: i32,
    pub event_name: String,
    pub event_guest: Option<String>,
    pub event_description: Option<String>,
    pub event_image: Option<String>,
    pub event_date: NaiveDate,
    pub event_start_time: Option<NaiveTime>,
    pub event_stop_time: Option<NaiveTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct NewEvent{
    pub restaurant_id: i32,
    pub restaurant_name: String,
    pub event_name: String,
    pub event_guest: Option<String>,
    pub event_description: Option<String>,
    pub event_image: Option<String>,
    pub event_date: NaiveDate,
    pub event_start_time: Option<NaiveTime>,
    pub event_stop_time: Option<NaiveTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}


