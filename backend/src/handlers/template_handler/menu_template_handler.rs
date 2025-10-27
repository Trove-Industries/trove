use axum::body::Body;
use axum::extract::{State};
use axum::response::{Html, IntoResponse};
use axum_extra::extract::CookieJar;
use http::{Request, StatusCode};
use crate::db::domains::subdomain::get_restaurant_name_from_domain;
use crate::services::template_services::menu_template_services::menu_template_service::get_full_menu_data;
use crate::state::AppState;
use crate::utils::domains::extract_subdomain;
use crate::utils::tera_engine::render_template;


pub async fn generate_menu_template(
    State(state): State<AppState>,
    jar: CookieJar,
    req: Request<Body>
) -> Result<impl IntoResponse,(StatusCode,String)> {
    
    let pool = state.pool;

    let subdomain = extract_subdomain(&req).unwrap_or_else(|| "default".to_string());

    let restaurant_name = get_restaurant_name_from_domain(&pool, &subdomain)
        .await
        .unwrap_or(None)
        .unwrap_or_else(|| "default".to_string());

    let full_data = get_full_menu_data(&pool, &restaurant_name,)
        .await
        .map_err(|(status, msg)| {
            eprintln!("Error fetching full menu data: {}", msg);
            (status, msg)
        })?;

    let html = render_template("menu.html.tera", &full_data)
        .map_err(|e| {
            eprintln!("Error rendering template: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Template error".to_string())
        })?;

    Ok(Html(html))
}


