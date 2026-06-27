use axum::{
    Router,
    body::Body,
    http::{HeaderValue, StatusCode, Uri, header},
    response::{Html, Response},
    routing::get,
};
use tower_http::set_header::SetResponseHeaderLayer;

use crate::assets::{StaticAssets, cache_control, content_type};
use crate::templates::render_index_html;

async fn home_page() -> Result<Html<String>, StatusCode> {
    render_index_html()
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn serve_static(uri: Uri) -> Result<Response, StatusCode> {
    let path = uri
        .path()
        .strip_prefix("/static/")
        .filter(|p| !p.is_empty() && !p.contains(".."))
        .ok_or(StatusCode::NOT_FOUND)?;

    let asset = StaticAssets::get(path).ok_or(StatusCode::NOT_FOUND)?;
    let body = Body::from(asset.data.into_owned());

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type(path))
        .header(header::CACHE_CONTROL, cache_control(path))
        .body(body)
        .expect("valid response"))
}

async fn favicon() -> Result<Response, StatusCode> {
    let asset = StaticAssets::get("sigma-favicon-32.png").ok_or(StatusCode::NOT_FOUND)?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/png")
        .header(header::CACHE_CONTROL, cache_control("sigma-favicon-32.png"))
        .body(Body::from(asset.data.into_owned()))
        .expect("valid response"))
}

/// Theme routes: home page, `/static/*`, and `/favicon.ico`.
pub fn router() -> Router {
    Router::new()
        .route("/", get(home_page))
        .route("/static/{*path}", get(serve_static))
        .route("/favicon.ico", get(favicon))
}

/// Same security headers used by the marketing site.
pub fn security_headers(router: Router) -> Router {
    router
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("content-security-policy"),
            HeaderValue::from_static(
                "default-src 'self'; base-uri 'self'; object-src 'none'; frame-ancestors 'none'; \
                 img-src 'self' data:; style-src 'self'; script-src 'self'; font-src 'self'; \
                 connect-src 'self'; form-action 'self'",
            ),
        ))
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn static_asset_served() {
        let app = router();
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/static/css/site.css")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn home_page_served() {
        let app = router();
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains("Sigma Tactical Group"));
    }
}
