use axum::Router;
use axum::routing::get;
use sqlx::PgPool;
use crate::handlers::template_handler::menu_template_handler::{generate_menu_template};

pub fn template_routes() ->Router<PgPool>{
    Router::new()
        .route("/", get(generate_menu_template))
        .route("/menu", get(generate_menu_template))
}
