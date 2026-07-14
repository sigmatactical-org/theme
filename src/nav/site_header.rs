//! [`SiteHeader`].

#[allow(unused_imports)]
use super::*;

/// Left side of the top bar: brand wordmark and optional breadcrumb trail.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SiteHeader {
    /// Wordmark next to the favicon (e.g. `Sigma Store`).
    pub brand: String,
    /// Brand link target (usually `/`).
    pub brand_href: String,
    /// Optional grey subtitle (legacy; prefer [`Self::breadcrumbs`]).
    pub menu_label: String,
    /// Where the user is within this service — earlier segments are links.
    pub breadcrumbs: Vec<Breadcrumb>,
}
impl SiteHeader {
    /// Minimal header: brand wordmark linking to `/`.
    #[must_use]
    pub fn new(brand: impl Into<String>) -> Self {
        Self {
            brand: brand.into(),
            brand_href: "/".into(),
            menu_label: String::new(),
            breadcrumbs: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_brand_href(mut self, href: impl Into<String>) -> Self {
        self.brand_href = href.into();
        self
    }

    #[must_use]
    pub fn with_menu_label(mut self, label: impl Into<String>) -> Self {
        self.menu_label = label.into();
        self
    }

    #[must_use]
    pub fn with_breadcrumb(mut self, crumb: Breadcrumb) -> Self {
        self.breadcrumbs.push(crumb);
        self
    }

    #[must_use]
    pub fn with_breadcrumbs(mut self, crumbs: impl IntoIterator<Item = Breadcrumb>) -> Self {
        self.breadcrumbs.extend(crumbs);
        self
    }

    /// Default marketing-site header.
    #[must_use]
    pub fn home() -> Self {
        Self::new("Sigma Tactical Group")
    }
}
