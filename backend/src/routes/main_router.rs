use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;
use crate::handlers::template_handler::menu_template_handler::generate_menu_template;
use crate::routes::admin_router::admin_router;
use crate::routes::auth_router::auth_routes;
use crate::routes::currency_router::currency_routes;
use crate::routes::menu_router::menu_routes;
use crate::routes::restaurant_router::restaurant_routes;
use crate::state::AppState;

pub fn main_router() -> Router<AppState> {
    let static_files = ServeDir::new("theme/minimalist/static");

    Router::new()

        // template routes
        .route("/", get(generate_menu_template))
        .route("/menu", get(generate_menu_template))

        .nest("/menu", menu_routes())
        .nest("/restaurant", restaurant_routes())
        .nest("/currency", currency_routes())
        .nest("/admin", admin_router())
        .nest("/auth", auth_routes())

        // .nest("/session", session_router())


        .nest_service("/static", static_files)
}

