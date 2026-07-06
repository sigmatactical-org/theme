//! Shared Sigma Tactical Group theme: copyright helpers, Askama templates, embedded
//! static assets, and optional axum / warp route helpers.
//!
//! **Branding is proprietary** — see `BRANDING.md`. Source code is MIT/Apache-2.0;
//! logos, wordmarks, and visual identity are not.

mod copyright;

#[cfg(feature = "embed")]
mod assets;

#[cfg(feature = "askama")]
pub mod errors;

#[cfg(feature = "askama")]
pub mod templates;

#[cfg(feature = "axum")]
pub mod axum;

#[cfg(feature = "warp")]
pub mod warp;

pub use copyright::{COPYRIGHT_START_YEAR, copyright_years, current_year};

/// Absolute path to on-disk static assets (for local dev / Docker COPY workflows).
#[must_use]
pub fn static_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/static")
}
