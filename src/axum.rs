//! Axum helpers: themed error responses, embedded asset routes, and the
//! shared security header set.

mod themed_panic_response;
pub use themed_panic_response::ThemedPanicResponse;

use axum::{
    Router,
    body::Body,
    http::{HeaderValue, StatusCode, Uri, header},
    response::Response,
    routing::get,
};
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::assets::{StaticAssets, cache_control, content_type};
use crate::errors::{internal_server_error_html, not_found_html};
use crate::security::{SECURITY_HEADERS, public_html_csp_production};

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

/// Themed 500 page for handler failures.
pub async fn internal_server_error() -> Response {
    internal_server_error_response()
}

pub(crate) fn internal_server_error_response() -> Response {
    html_response(
        StatusCode::INTERNAL_SERVER_ERROR,
        internal_server_error_html(),
    )
}

/// Panic-catching middleware that renders the themed 500 page.
pub fn catch_panic_layer() -> CatchPanicLayer<ThemedPanicResponse> {
    CatchPanicLayer::custom(ThemedPanicResponse)
}

/// Response for an embedded asset: body plus content-type and cache-control.
fn embedded_response(path: &str) -> Result<Response, StatusCode> {
    let asset = StaticAssets::get(path).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type(path))
        .header(header::CACHE_CONTROL, cache_control(path))
        .body(Body::from(asset.data.into_owned()))
        .expect("valid response"))
}

async fn serve_static(uri: Uri) -> Result<Response, StatusCode> {
    let path = uri
        .path()
        .strip_prefix("/static/")
        .filter(|p| !p.is_empty() && !p.contains(".."))
        .ok_or(StatusCode::NOT_FOUND)?;
    embedded_response(path)
}

async fn favicon() -> Result<Response, StatusCode> {
    embedded_response("sigma-favicon-32.png")
}

/// Theme asset routes: `/static/*` and `/favicon.ico`. The service provides
/// its own landing page at `/`.
pub fn asset_router() -> Router {
    Router::new()
        .route("/static/{*path}", get(serve_static))
        .route("/favicon.ico", get(favicon))
}

/// Apply the shared security header set (see [`crate::SECURITY_HEADERS`]) and
/// the production CSP to every response, matching the warp helper.
///
/// `connect_src_extra` is appended to the CSP `connect-src` (e.g. the
/// identity BFF origin); pass `""` when nothing beyond `'self'` is needed.
#[must_use]
pub fn security_headers(router: Router, connect_src_extra: &str) -> Router {
    let csp = public_html_csp_production(connect_src_extra, false);
    let mut router = router.layer(SetResponseHeaderLayer::if_not_present(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_str(&csp).expect("valid CSP header value"),
    ));
    for (name, value) in SECURITY_HEADERS {
        router = router.layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static(name),
            HeaderValue::from_static(value),
        ));
    }
    router
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    async fn get_response(app: Router, uri: &str) -> Response {
        app.oneshot(
            axum::http::Request::builder()
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn static_asset_served() {
        let response = get_response(asset_router(), "/static/css/site.css").await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn favicon_served_with_cache_control() {
        let response = get_response(asset_router(), "/favicon.ico").await;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "image/png"
        );
        assert!(response.headers().contains_key(header::CACHE_CONTROL));
    }

    #[tokio::test]
    async fn asset_router_omits_home_page() {
        let response = get_response(asset_router(), "/").await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn fallback_renders_themed_404() {
        let app = asset_router().fallback(not_found);
        let response = get_response(app, "/does-not-exist").await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let html = std::str::from_utf8(&body).unwrap();
        assert!(html.contains("Oops"));
    }

    #[tokio::test]
    async fn security_headers_apply_unified_set() {
        let app = security_headers(asset_router(), "http://identity.example");
        let response = get_response(app, "/favicon.ico").await;
        let headers = response.headers();
        let csp = headers
            .get("content-security-policy")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(csp.contains("worker-src 'self' blob: data:"));
        assert!(csp.contains("connect-src 'self' http://identity.example"));
        assert!(csp.contains("upgrade-insecure-requests"));
        for (name, value) in SECURITY_HEADERS {
            assert_eq!(headers.get(*name).unwrap(), value, "header {name}");
        }
    }
}
