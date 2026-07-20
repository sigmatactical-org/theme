//! Warp helpers: the standard site bootstrap (listen address, graceful serve,
//! security headers, shared route scaffold) plus themed asset and error
//! filters.

mod template_error;
pub use template_error::TemplateError;

use std::convert::Infallible;
use std::fmt::Display;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::OnceLock;

use warp::http::header::{HeaderMap, HeaderName, HeaderValue};
use warp::http::{StatusCode, header};
use warp::{Filter, Rejection, Reply};

use crate::assets::{StaticAssets, cache_control, content_type};
use crate::errors::{internal_server_error_html, method_not_allowed_html, not_found_html};
use crate::security::{CspOptions, SECURITY_HEADERS, public_html_csp_with};

/// Listen address from the `PORT` environment variable (default **8080**).
/// Binds IPv4 **`0.0.0.0`**.
#[must_use]
pub fn listen_addr_from_env() -> SocketAddr {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port)
}

/// Resolves when the process receives a shutdown signal (SIGTERM/SIGINT on
/// Unix, Ctrl-C elsewhere). Cloud Run sends SIGTERM before stopping an
/// instance, so this lets warp drain in-flight requests. If signal handlers
/// can't be installed we never resolve, keeping the server running rather
/// than shutting down spuriously.
async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        let (mut term, mut int) = match (
            signal(SignalKind::terminate()),
            signal(SignalKind::interrupt()),
        ) {
            (Ok(term), Ok(int)) => (term, int),
            _ => {
                eprintln!("warning: could not install signal handlers; graceful shutdown disabled");
                std::future::pending::<()>().await;
                return;
            }
        };
        tokio::select! {
            _ = term.recv() => {}
            _ = int.recv() => {}
        }
    }

    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}

/// Bind `addr`, print the startup banner (`{name} listening on http://{addr}`),
/// and serve `routes` until a shutdown signal arrives (graceful drain).
///
/// # Errors
///
/// Returns an error when binding the listener fails (e.g. port in use).
pub async fn serve<F, R>(name: &str, addr: SocketAddr, routes: F) -> std::io::Result<()>
where
    F: Filter<Extract = (R,), Error = Infallible> + Clone + Send + Sync + 'static,
    R: Reply,
{
    // Bind explicitly so a failure (e.g. port in use) is a returned error, not a panic.
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("{name} listening on http://{addr}");
    warp::serve(routes)
        .incoming(listener)
        .graceful(shutdown_signal())
        .run()
        .await;
    Ok(())
}

fn security_header_map(options: &CspOptions) -> HeaderMap {
    let mut map = HeaderMap::new();
    map.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_str(&public_html_csp_with(options)).expect("valid CSP header value"),
    );
    for (name, value) in SECURITY_HEADERS {
        map.insert(HeaderName::from_static(name), HeaderValue::from_static(value));
    }
    map
}

/// Apply the shared security header set (see [`crate::SECURITY_HEADERS`]) and
/// the production CSP to every reply, matching the axum helper.
///
/// `connect_src_extra` is appended to the CSP `connect-src` (e.g. the
/// identity BFF origin); pass `""` when nothing beyond `'self'` is needed.
pub fn security_headers<F, R>(
    routes: F,
    connect_src_extra: impl Into<String>,
) -> impl Filter<Extract = (impl Reply,), Error = F::Error> + Clone + Send + 'static
where
    F: Filter<Extract = (R,)> + Clone + Send + Sync + 'static,
    R: Reply,
{
    security_headers_with(routes, CspOptions::production(connect_src_extra))
}

/// [`security_headers`] with per-service CSP adjustments — a cross-origin
/// `form-action`, inline styles, and so on. See [`CspOptions`].
pub fn security_headers_with<F, R>(
    routes: F,
    options: CspOptions,
) -> impl Filter<Extract = (impl Reply,), Error = F::Error> + Clone + Send + 'static
where
    F: Filter<Extract = (R,)> + Clone + Send + Sync + 'static,
    R: Reply,
{
    routes.with(warp::reply::with::headers(security_header_map(&options)))
}

/// Standard site filter chain shared by the Sigma warp services:
/// `GET /up`, the service's `extra` routes (e.g. sigma-pg health routes),
/// the service's `index` filter, `/static/*`, `/favicon.ico`, and themed
/// 404/405/500 rejection recovery. Wrap the result with [`security_headers`]
/// before passing it to [`serve`].
pub fn site_routes<I, IR, E, ER>(
    index: I,
    extra: E,
) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone + Send + 'static
where
    I: Filter<Extract = (IR,), Error = Rejection> + Clone + Send + Sync + 'static,
    IR: Reply + Send,
    E: Filter<Extract = (ER,), Error = Rejection> + Clone + Send + Sync + 'static,
    ER: Reply + Send,
{
    warp::path("up")
        .and(warp::get())
        .map(|| warp::reply::with_status("up", StatusCode::OK))
        .or(extra)
        .or(index)
        .or(static_files())
        .or(favicon())
        .recover(handle_rejection)
}

fn cached_page_body<E: Display>(
    cache: &'static OnceLock<String>,
    render: impl FnOnce() -> Result<String, E>,
) -> Result<&'static str, ()> {
    if let Some(body) = cache.get() {
        return Ok(body);
    }
    match render() {
        // `get_or_init` in case another request rendered concurrently.
        Ok(body) => Ok(cache.get_or_init(|| body)),
        Err(err) => {
            tracing::error!(%err, "page render failed; serving 500 and re-rendering next request");
            Err(())
        }
    }
}

/// `GET /` filter serving a page rendered once and reused for the process
/// lifetime. Only successful renders are cached: a failed render is logged,
/// rejected (recovered to the themed 500 page by [`handle_rejection`]), and
/// re-rendered on the next request.
pub fn cached_page<F, E>(
    cache: &'static OnceLock<String>,
    render: F,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone
where
    F: Fn() -> Result<String, E> + Clone + Send + Sync + 'static,
    E: Display,
{
    warp::get().and(warp::path::end()).and_then(move || {
        let render = render.clone();
        async move {
            match cached_page_body(cache, render) {
                Ok(body) => Ok(warp::reply::html(body)),
                Err(()) => Err(warp::reject::custom(TemplateError)),
            }
        }
    })
}

fn embedded_response(path: &str, data: Vec<u8>) -> warp::reply::Response {
    let mut resp = warp::reply::Response::new(data.into());
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        content_type(path).parse().expect("valid content-type"),
    );
    resp.headers_mut().insert(
        header::CACHE_CONTROL,
        cache_control(path).parse().expect("valid cache-control"),
    );
    resp
}

/// Static asset filter (`GET /static/*`).
pub fn static_files() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("static")
        .and(warp::path::tail())
        .and_then(|tail: warp::path::Tail| async move {
            let path = tail.as_str();
            if path.is_empty() || path.contains("..") {
                return Err(warp::reject::not_found());
            }
            StaticAssets::get(path)
                .map(|asset| embedded_response(path, asset.data.into_owned()))
                .ok_or_else(warp::reject::not_found)
        })
}

/// Favicon filter (`GET /favicon.ico`).
pub fn favicon() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path("favicon.ico")
        .and(warp::path::end())
        .and_then(|| async {
            StaticAssets::get("sigma-favicon-32.png")
                .map(|asset| embedded_response("sigma-favicon-32.png", asset.data.into_owned()))
                .ok_or_else(warp::reject::not_found)
        })
}

/// Warp rejection handler for themed 404/405/500 responses.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        return Ok(warp::reply::with_status(
            warp::reply::html(not_found_html()),
            StatusCode::NOT_FOUND,
        ));
    }

    if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        return Ok(warp::reply::with_status(
            warp::reply::html(method_not_allowed_html()),
            StatusCode::METHOD_NOT_ALLOWED,
        ));
    }

    Ok(warp::reply::with_status(
        warp::reply::html(internal_server_error_html()),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    fn extra_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
        warp::path("extra").and(warp::get()).map(|| "extra ok")
    }

    fn test_site() -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
        static BODY: OnceLock<String> = OnceLock::new();
        let index = cached_page(&BODY, || Ok::<_, askama::Error>("<p>home</p>".to_string()));
        security_headers(site_routes(index, extra_route()), "http://identity.example")
    }

    #[tokio::test]
    async fn site_routes_up_returns_200() {
        let res = warp::test::request()
            .method("GET")
            .path("/up")
            .reply(&test_site())
            .await;
        assert_eq!(res.status(), 200);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "up");
    }

    #[tokio::test]
    async fn site_routes_serves_index_and_extra() {
        let res = warp::test::request()
            .method("GET")
            .path("/")
            .reply(&test_site())
            .await;
        assert_eq!(res.status(), 200);
        assert_eq!(std::str::from_utf8(res.body()).unwrap(), "<p>home</p>");

        let res = warp::test::request()
            .method("GET")
            .path("/extra")
            .reply(&test_site())
            .await;
        assert_eq!(res.status(), 200);
    }

    #[tokio::test]
    async fn site_routes_sets_unified_security_headers() {
        let res = warp::test::request()
            .method("GET")
            .path("/")
            .reply(&test_site())
            .await;
        let headers = res.headers();
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

    #[tokio::test]
    async fn site_routes_unknown_returns_404_html() {
        let res = warp::test::request()
            .method("GET")
            .path("/missing-page")
            .reply(&test_site())
            .await;
        assert_eq!(res.status(), 404);
        let body = std::str::from_utf8(res.body()).expect("utf-8");
        assert!(body.contains("Oops"));
    }

    #[tokio::test]
    async fn site_routes_wrong_method_returns_405() {
        let res = warp::test::request()
            .method("POST")
            .path("/")
            .reply(&test_site())
            .await;
        assert_eq!(res.status(), 405);
        let body = std::str::from_utf8(res.body()).expect("utf-8");
        assert!(body.contains("Method not allowed"));
    }

    #[tokio::test]
    async fn static_assets_get_cache_control() {
        let res = warp::test::request()
            .method("GET")
            .path("/static/vendor/bootstrap-5.3.3/bootstrap.min.css")
            .reply(&test_site())
            .await;
        assert_eq!(res.status(), 200);
        assert_eq!(
            res.headers().get("cache-control").unwrap(),
            "public, max-age=31536000, immutable"
        );
    }

    #[tokio::test]
    async fn handle_rejection_renders_500_for_non_not_found() {
        let reply = handle_rejection(warp::reject::custom(TemplateError))
            .await
            .expect("recovery is infallible");
        let resp = Reply::into_response(reply);
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn cached_page_body_caches_only_successful_renders() {
        static CACHE: OnceLock<String> = OnceLock::new();
        static CALLS: AtomicUsize = AtomicUsize::new(0);

        let failing = || {
            CALLS.fetch_add(1, Ordering::SeqCst);
            Err::<String, _>(askama::Error::Fmt)
        };
        assert!(cached_page_body(&CACHE, failing).is_err());
        assert!(CACHE.get().is_none(), "failed render must not be cached");

        let ok = || {
            CALLS.fetch_add(1, Ordering::SeqCst);
            Ok::<_, askama::Error>("<p>cached</p>".to_string())
        };
        assert_eq!(cached_page_body(&CACHE, ok).unwrap(), "<p>cached</p>");
        assert_eq!(cached_page_body(&CACHE, ok).unwrap(), "<p>cached</p>");
        assert_eq!(
            CALLS.load(Ordering::SeqCst),
            2,
            "render runs once per failure and once for the cached success"
        );
    }

    #[test]
    fn listen_addr_defaults_to_8080() {
        // PORT is not set in the test environment.
        assert_eq!(listen_addr_from_env().to_string(), "0.0.0.0:8080");
    }
}
