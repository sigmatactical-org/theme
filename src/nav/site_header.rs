//! [`SiteHeader`].

#[allow(unused_imports)]
use super::*;

/// Left side of the top bar: the fixed sigma symbol with the current area's
/// wordmark, the left-aligned site menu, and the breadcrumb trail rendered
/// in a bar under the navbar.
///
/// The sigma brand icon is fixed in `assets/templates/base.html` so it is
/// identical on every site; services choose the wordmark next to it (the
/// area the visitor is in — e.g. `Identity`, `Store`, `Contact Us`), the
/// brand link target, the menu (usually [`site_menu`]) and the breadcrumbs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SiteHeader {
    /// Brand link target (usually `/`).
    pub brand_href: String,
    /// Wordmark next to the sigma symbol: the area the visitor is in
    /// (e.g. `Store`). Defaults to `Sigma Tactical Group`.
    pub brand_label: String,
    /// Left-aligned site menu entries (usually [`site_menu`]).
    pub menu: Vec<MenuItem>,
    /// Where the user is within this service — earlier segments are links.
    pub breadcrumbs: Vec<Breadcrumb>,
}
impl SiteHeader {
    /// Minimal header: brand linking to `/` with the default wordmark, no
    /// menu, no breadcrumbs.
    #[must_use]
    pub fn new() -> Self {
        Self {
            brand_href: "/".into(),
            brand_label: "Sigma Tactical Group".into(),
            menu: Vec::new(),
            breadcrumbs: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_brand_href(mut self, href: impl Into<String>) -> Self {
        self.brand_href = href.into();
        self
    }

    /// Set the wordmark to the area the visitor is in (e.g. `Accounting`).
    #[must_use]
    pub fn with_brand_label(mut self, label: impl Into<String>) -> Self {
        self.brand_label = label.into();
        self
    }

    #[must_use]
    pub fn with_menu(mut self, menu: impl IntoIterator<Item = MenuItem>) -> Self {
        self.menu.extend(menu);
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

    /// Default header with the standard site menu and nothing highlighted.
    #[must_use]
    pub fn home() -> Self {
        Self::new().with_menu(site_menu(None))
    }
}
