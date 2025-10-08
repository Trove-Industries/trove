use axum::extract::State;
use sqlx::PgPool;

pub async fn create_event(
    State(pool): State<PgPool>,

){}

pub async fn get_event(){}