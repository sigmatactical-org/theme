//! Themed HTTP error page HTML (404, 500, 403, 405) with plain-text fallbacks when templates fail.

use crate::templates::{
    render_forbidden_html, render_internal_server_error_html, render_method_not_allowed_html,
    render_not_found_html,
};

/// Minimal HTML used when Askama rendering fails.
pub mod fallbacks {
    pub const NOT_FOUND: &str = "<!DOCTYPE html><html lang=\"en\"><meta charset=\"utf-8\"><title>Page not found</title><p>Not found.</p>";
    pub const INTERNAL_SERVER_ERROR: &str = "<!DOCTYPE html><html lang=\"en\"><meta charset=\"utf-8\"><title>Error</title><p>Internal Server Error.</p>";
    pub const FORBIDDEN: &str = "<!DOCTYPE html><html lang=\"en\"><meta charset=\"utf-8\"><title>Access denied</title><p>Forbidden.</p>";
    pub const METHOD_NOT_ALLOWED: &str = "<!DOCTYPE html><html lang=\"en\"><meta charset=\"utf-8\"><title>Method not allowed</title><p>Method not allowed.</p>";
}

/// Render the themed 404 page, or a minimal fallback.
#[must_use]
pub fn not_found_html() -> String {
    render_not_found_html().unwrap_or_else(|_| fallbacks::NOT_FOUND.to_string())
}

/// Render the themed 500 page, or a minimal fallback.
#[must_use]
pub fn internal_server_error_html() -> String {
    render_internal_server_error_html()
        .unwrap_or_else(|_| fallbacks::INTERNAL_SERVER_ERROR.to_string())
}

/// Render the themed 403 page, or a minimal fallback.
#[must_use]
pub fn forbidden_html() -> String {
    render_forbidden_html().unwrap_or_else(|_| fallbacks::FORBIDDEN.to_string())
}

/// Render the themed 405 page, or a minimal fallback.
#[must_use]
pub fn method_not_allowed_html() -> String {
    render_method_not_allowed_html().unwrap_or_else(|_| fallbacks::METHOD_NOT_ALLOWED.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_error_pages_render_themed_html() {
        assert!(not_found_html().contains("Oops"));
        assert!(internal_server_error_html().contains("Something went wrong"));
        assert!(forbidden_html().contains("Access denied"));
        assert!(method_not_allowed_html().contains("Method not allowed"));
    }
}
