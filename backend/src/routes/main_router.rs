use axum::Router;
use axum::routing::get;
use sqlx::PgPool;
use tower_http::services::ServeDir;
use crate::handlers::template_handler::menu_template_handler::generate_menu_template;
use crate::routes::menu_router::menu_routes;
use crate::routes::restaurant_router::restaurant_routes;

pub fn main_router() -> Router<PgPool> {
    let static_files = ServeDir::new("theme/dev/static");

    Router::new()

        // template routes
        .route("/", get(generate_menu_template))
        .route("/menu", get(generate_menu_template))

        .nest("/menu", menu_routes())
        .nest("/restaurant", restaurant_routes())

        .nest_service("/static", static_files)
}

