use std::convert::Infallible;

use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

use crate::assets::{StaticAssets, cache_control, content_type};
use crate::templates::{
    render_index_html, render_internal_server_error_html, render_not_found_html,
};

fn embedded_response(path: &str, data: Vec<u8>) -> warp::reply::Response {
    let mut resp = warp::reply::Response::new(data.into());
    resp.headers_mut().insert(
        warp::http::header::CONTENT_TYPE,
        content_type(path).parse().expect("valid content-type"),
    );
    resp.headers_mut().insert(
        warp::http::header::CACHE_CONTROL,
        cache_control(path).parse().expect("valid cache-control"),
    );
    resp
}

#[derive(Debug)]
struct TemplateError;
impl warp::reject::Reject for TemplateError {}

/// Home page filter (`GET /`).
pub fn index() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::get().and(warp::path::end()).and_then(|| async {
        render_index_html()
            .map(warp::reply::html)
            .map_err(|_| warp::reject::custom(TemplateError))
    })
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

const FALLBACK_404: &str = "<!DOCTYPE html><html lang=\"en\"><meta charset=\"utf-8\"><title>Page not found</title><p>Not found.</p>";
const FALLBACK_405: &str = "<!DOCTYPE html><html lang=\"en\"><meta charset=\"utf-8\"><title>Method not allowed — Sigma Tactical Group</title><p>That method isn’t allowed here.</p>";
const FALLBACK_500: &str = "<!DOCTYPE html><html lang=\"en\"><meta charset=\"utf-8\"><title>Error</title><p>Internal Server Error.</p>";

/// Warp rejection handler for themed 404/500 responses.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        let reply = match render_not_found_html() {
            Ok(body) => warp::reply::with_status(warp::reply::html(body), StatusCode::NOT_FOUND),
            Err(_) => warp::reply::with_status(
                warp::reply::html(String::from(FALLBACK_404)),
                StatusCode::NOT_FOUND,
            ),
        };
        return Ok(reply);
    }

    if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        return Ok(warp::reply::with_status(
            warp::reply::html(String::from(FALLBACK_405)),
            StatusCode::METHOD_NOT_ALLOWED,
        ));
    }

    let body = match render_internal_server_error_html() {
        Ok(html) => html,
        Err(_) => String::from(FALLBACK_500),
    };
    Ok(warp::reply::with_status(
        warp::reply::html(body),
        StatusCode::INTERNAL_SERVER_ERROR,
    ))
}

/// Full site filter: `/`, `/static/*`, `/favicon.ico`, security headers, plus rejection recovery.
pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone + Send + 'static
{
    use warp::reply::with::header;

    index()
        .or(static_files())
        .or(favicon())
        .recover(handle_rejection)
        .with(header(
            "content-security-policy",
            "default-src 'self'; base-uri 'self'; object-src 'none'; frame-ancestors 'none'; \
             img-src 'self' data:; style-src 'self'; script-src 'self'; font-src 'self'; \
             connect-src 'self'; form-action 'self'; upgrade-insecure-requests",
        ))
        .with(header("x-content-type-options", "nosniff"))
        .with(header("x-frame-options", "DENY"))
        .with(header("referrer-policy", "strict-origin-when-cross-origin"))
        .with(header("cross-origin-opener-policy", "same-origin"))
        .with(header(
            "permissions-policy",
            "geolocation=(), microphone=(), camera=()",
        ))
        .with(header(
            "strict-transport-security",
            "max-age=63072000; includeSubDomains; preload",
        ))
}
