use sqlx::PgPool;
use crate::db::template_queries::events_template_queries::{get_all_events, new_event};
use crate::models::template_models::events_template_models::{Events, NewEvent};

pub async fn create_event(
    poll: &PgPool,
    new: NewEvent,
) -> Result<Events, sqlx::Error>{
    new_event(poll,new).await
}

pub async fn get_events (
    pool: &PgPool,
    restaurant_id: i32,
) ->Result<Vec<Events>, sqlx::Error>{
    get_all_events(pool, restaurant_id).await
}