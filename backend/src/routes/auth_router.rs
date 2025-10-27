use axum::{Router, routing::post};
use crate::handlers::auth_handler::login::login_handler;
use crate::handlers::auth_handler::signup::signup_handler;
use crate::state::AppState;

pub fn auth_routes() -> Router<AppState>{
    Router::new()
        .route("/signup", post(signup_handler))
        .route("/login", post(login_handler))
}

