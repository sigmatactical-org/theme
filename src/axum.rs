use std::any::Any;

use axum::{
    Router,
    body::Body,
    http::{HeaderValue, StatusCode, Uri, header},
    response::{Html, Response},
    routing::get,
};
use tower_http::catch_panic::{CatchPanicLayer, ResponseForPanic};
use tower_http::set_header::SetResponseHeaderLayer;

use crate::assets::{StaticAssets, cache_control, content_type};
use crate::errors::{
    forbidden_html, internal_server_error_html, method_not_allowed_html, not_found_html,
};
use crate::templates::render_index_html;

fn html_response(status: StatusCode, body: String) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from(body))
        .expect("valid response")
}

/// Themed 404 page for unmatched routes.
pub async fn not_found() -> Response {
    html_response(StatusCode::NOT_FOUND, not_found_html())
}

/// Themed 403 page for forbidden requests.
pub async fn forbidden() -> Response {
    html_response(StatusCode::FORBIDDEN, forbidden_html())
}

/// Themed 500 page for handler failures.
pub async fn internal_server_error() -> Response {
    internal_server_error_response()
}

fn internal_server_error_response() -> Response {
    html_response(
        StatusCode::INTERNAL_SERVER_ERROR,
        internal_server_error_html(),
    )
}

/// Themed 405 page for disallowed HTTP methods.
pub async fn method_not_allowed() -> Response {
    html_response(StatusCode::METHOD_NOT_ALLOWED, method_not_allowed_html())
}

/// Panic handler that returns the themed 500 page instead of an empty response.
#[derive(Clone, Copy, Debug)]
pub struct ThemedPanicResponse;

impl ResponseForPanic for ThemedPanicResponse {
    type ResponseBody = Body;

    fn response_for_panic(
        &mut self,
        _err: Box<dyn Any + Send + 'static>,
    ) -> Response<Self::ResponseBody> {
        internal_server_error_response()
    }
}

/// Panic-catching middleware that renders the themed 500 page.
pub fn catch_panic_layer() -> CatchPanicLayer<ThemedPanicResponse> {
    CatchPanicLayer::custom(ThemedPanicResponse)
}

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

/// Theme routes without the generic home page: `/static/*` and `/favicon.ico`.
///
/// Use this instead of [`router`] when the service provides its own themed
/// landing page at `/` (avoids an axum route conflict on `GET /`).
pub fn asset_router() -> Router {
    Router::new()
        .route("/static/{*path}", get(serve_static))
        .route("/favicon.ico", get(favicon))
}

/// Theme routes: home page, `/static/*`, and `/favicon.ico`.
pub fn router() -> Router {
    asset_router().route("/", get(home_page))
}

/// Theme routes plus a fallback 404 handler for unmatched paths.
pub fn router_with_fallback() -> Router {
    router().fallback(not_found)
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
                 worker-src 'self' blob: data:; connect-src 'self'; form-action 'self'",
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
    async fn asset_router_omits_home_page() {
        let app = asset_router();
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
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

    #[tokio::test]
    async fn fallback_renders_themed_404() {
        let app = router_with_fallback();
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains("Oops"));
    }
}
