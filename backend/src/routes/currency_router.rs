use axum::Router;
use axum::routing::{get, post};
use sqlx::PgPool;
use crate::handlers::currency_handler::currency::{create_currency_handler, get_all_currencies_handler, get_currency_by_iso_handler};

pub fn currency_routes() ->Router<PgPool>{
    Router::new()
        .route("/create-currency", post(create_currency_handler))
        .route("/get-currency", get(get_all_currencies_handler))
        .route("/get-currency/{id}", get(get_currency_by_iso_handler))

}