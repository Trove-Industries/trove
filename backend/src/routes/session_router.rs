use axum::{routing::post, Router};
use axum::routing::get;
use sqlx::PgPool;
use crate::handlers::session_handler::restore::restore_session_handler;
use crate::handlers::session_handler::session::{create_session_handler};

pub fn session_router() -> Router<PgPool> {
    Router::new()
        //.route("/session/create", post(create_session_handler))
}
