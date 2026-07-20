//! [`NavEntry`].

/// Link to a content page in a service's own navigation list (sidebar or
/// tab strip): the page slug plus its display title.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavEntry {
    /// URL path segment identifying the page (e.g. `getting-started`).
    pub slug: String,
    /// Human-readable link label.
    pub title: String,
}
