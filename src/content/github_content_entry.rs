//! [`GithubContentEntry`].

use serde::Deserialize;

/// One entry of a GitHub contents-API directory listing.
#[derive(Debug, Deserialize)]
pub struct GithubContentEntry {
    /// File name within the listed directory (e.g. `welcome.md`).
    pub name: String,
    /// Raw download URL; `None` for entries without one (e.g. directories).
    pub download_url: Option<String>,
}
