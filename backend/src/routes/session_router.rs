use axum::{ Router};
use sqlx::PgPool;


pub fn session_router() -> Router<PgPool> {
    Router::new()
        //.route("/session/create", post(create_session_handler))
}
