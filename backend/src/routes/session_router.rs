use axum::{routing::post, Router};
use sqlx::PgPool;
use crate::handlers::session_handler::session::start_trial_session;

pub fn session_router() -> Router<PgPool> {
    Router::new()
        .route("/start-trial", post(start_trial_session))
}
