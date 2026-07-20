//! [`SiteHeader`].

use super::{Breadcrumb, MenuItem, site_menu};

/// Left side of the top bar: the fixed sigma symbol with the current area's
/// wordmark, the left-aligned site menu, and the breadcrumb trail rendered
/// in a bar under the navbar.
///
/// The sigma brand icon is fixed in `assets/templates/base.html` so it is
/// identical on every site; every service names the area the visitor is in
/// (the wordmark next to the icon — e.g. `Identity`, `Store`, `Contact Us`)
/// and chooses the brand link target, the menu (usually [`site_menu`]) and
/// the breadcrumbs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SiteHeader {
    /// Brand link target (usually `/`).
    pub brand_href: String,
    /// Wordmark next to the sigma symbol: the area the visitor is in
    /// (e.g. `Store`). Required — every site names its own area.
    pub brand_label: String,
    /// Left-aligned site menu entries (usually [`site_menu`]).
    pub menu: Vec<MenuItem>,
    /// Where the user is within this service — earlier segments are links.
    pub breadcrumbs: Vec<Breadcrumb>,
}
impl SiteHeader {
    /// Minimal header: brand linking to `/` with this area's wordmark
    /// (e.g. `Store`), no menu, no breadcrumbs.
    #[must_use]
    pub fn new(brand_label: impl Into<String>) -> Self {
        Self {
            brand_href: "/".into(),
            brand_label: brand_label.into(),
            menu: Vec::new(),
            breadcrumbs: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_brand_href(mut self, href: impl Into<String>) -> Self {
        self.brand_href = href.into();
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

    /// Marketing-site header: the Sigma Tactical Group wordmark with the
    /// standard site menu and nothing highlighted.
    #[must_use]
    pub fn home() -> Self {
        Self::new("Sigma Tactical Group").with_menu(site_menu(None))
    }
}
