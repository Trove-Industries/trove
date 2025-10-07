use axum::body::Body;
use axum::extract::{Path, State};
use axum::{Error, Json};
use axum::response::{Html, IntoResponse, Response};
use http::{Request, StatusCode};
use sqlx::PgPool;
use crate::models::template_models::FullData;
use crate::services::template_service::{get_restaurant_with_menu};
use crate::utils::domains::extract_subdomain;
use crate::utils::tera_engine::render_template;


pub async fn generate_menu_template(
    State(pool): State<PgPool>,
    req: Request<Body>
) -> Result<impl IntoResponse,(StatusCode,String)> {

    let subdomain = extract_subdomain(&req).unwrap_or_else(|| "default".to_string());

    println!("Looking up restaurant with subdomain: {}", subdomain);
    let full_data = get_restaurant_with_menu(&pool, &subdomain)
        .await
        .map_err(|e| {
            eprintln!("Error fetching data: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
        })?;

    let html = render_template("menu.html.tera", &full_data)
        .map_err(|e| {
            eprintln!("Error rendering template: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error".to_string())
        })?;

    Ok(Html(html))
}


