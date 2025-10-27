use axum::{Router};
use axum::routing::get;
use crate::admin::admin_handler::admin_handler::get_all_users_handler;
use crate::state::AppState;

pub fn admin_router() -> Router<AppState> {
    Router::new()
        //.route("/cleanup-sessions", post(admin_cleanup_sessions_handler))
        .route("/get-all-users", get(get_all_users_handler))
}
