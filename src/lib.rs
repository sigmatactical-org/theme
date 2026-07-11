//! Shared Sigma Tactical Group theme: copyright helpers, Askama templates, embedded
//! static assets, and optional axum / warp route helpers.
//!
//! **Brand and artwork are proprietary** — see `BRANDING.md`. Source code is MIT/Apache-2.0;
//! logos, wordmarks, artwork, and visual identity are not.

mod copyright;
mod security;

#[cfg(feature = "embed")]
mod assets;

#[cfg(feature = "askama")]
pub mod errors;

#[cfg(feature = "askama")]
pub mod nav;

#[cfg(feature = "site-nav")]
pub mod site_nav;

#[cfg(feature = "askama")]
pub mod templates;

#[cfg(feature = "axum")]
pub mod axum;

#[cfg(feature = "warp")]
pub mod warp;

pub use copyright::{COPYRIGHT_START_YEAR, copyright_years, current_year};
pub use security::{CSP_ALTCHA, public_html_csp, public_html_csp_production};

/// Absolute path to on-disk static assets (for local dev / Docker COPY workflows).
#[must_use]
pub fn static_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/static")
}
