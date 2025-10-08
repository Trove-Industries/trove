use axum::Error;
use sqlx::{Execute, PgPool};
use crate::models::template_models::events_template_models::{Events, NewEvent};

pub async fn new_event(
    pool: &PgPool,
    new_event: NewEvent,
) ->Result<Events, sqlx::Error>{

    let new_event = sqlx::query_as(
        r#"
                INSERT INTO events (restaurant_id, event_name, event_guest, event_description, event_image, event_date, event_start_time, event_stop_time)
                VALUES (
                        (SELECT id FROM restaurants WHERE restaurant_name = $1)
                        $2, $3 $4, $5, $6, $7, $8, $9
                )
                RETURNING id, restaurant_id, event_name, event_guest, event_description, event_image, event_date, event_start_time, event_stop_time
            "#
    )
        .bind(new_event.restaurant_name)
        .bind(new_event.restaurant_id)
        .bind(new_event.event_name)
        .bind(new_event.event_guest)
        .bind(new_event.event_description)
        .bind(new_event.event_image)
        .bind(new_event.event_date)
        .bind(new_event.event_start_time)
        .bind(new_event.event_stop_time)
        .persistent(false)
        .fetch_one(pool)
        .await?;

    Ok(new_event)
}
pub async fn get_all_events(
    pool: &PgPool,
    restaurant_id: i32,
) ->Result<Vec<Events>, sqlx::Error>{

    let all_events = sqlx::query_as(
        r#"
                SELECT *
                FROM events
                WHERE restaurant_id = $1
            "#
    )
        .bind(restaurant_id)
        .persistent(false)
        .fetch_all(pool)
        .await?;

        Ok(all_events)
}