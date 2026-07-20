//! [`ContentError`].

use thiserror::Error;

/// Failure while listing or downloading GitHub-hosted content.
#[derive(Debug, Error)]
pub enum ContentError {
    /// The HTTP request itself failed (connect, decode, non-success download).
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    /// GitHub answered the listing request with a non-success status.
    #[error("content request failed: {0}")]
    Request(String),
}
