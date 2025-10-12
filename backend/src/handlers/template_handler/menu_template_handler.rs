use axum::body::Body;
use axum::extract::{Path, State};
use axum::{Error, Json};
use axum::response::{Html, IntoResponse, Response};
use http::{Request, StatusCode};
use sqlx::PgPool;
use crate::db::domains::subdomain::get_restaurant_name_from_domain;
use crate::models::template_models::menu_template_models::FullData;
use crate::services::template_services::menu_template_service::{get_restaurant_with_menu};
use crate::services::template_services::menu_template_services::menu_template_service::get_full_menu_data;
use crate::utils::domains::extract_subdomain;
use crate::utils::tera_engine::render_template;


pub async fn generate_menu_template(
    State(pool): State<PgPool>,
    req: Request<Body>
) -> Result<impl IntoResponse,(StatusCode,String)> {

    let subdomain = extract_subdomain(&req).unwrap_or_else(|| "default".to_string());

    let restaurant_name = get_restaurant_name_from_domain(&pool, &subdomain)
        .await
        .unwrap_or(None)
        .unwrap_or_else(|| "default".to_string());

    let full_data = get_full_menu_data(&pool, &restaurant_name).await;


    let html = render_template("menu.html.tera", &full_data)
        .map_err(|e| {
            eprintln!("Error rendering template: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error".to_string())
        })?;

    Ok(Html(html))
}


