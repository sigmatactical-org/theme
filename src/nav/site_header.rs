//! [`SiteHeader`].

#[allow(unused_imports)]
use super::*;

/// Left side of the top bar: the fixed Sigma Tactical Group brand, the
/// left-aligned site menu, and the breadcrumb trail rendered in a bar under
/// the navbar.
///
/// The brand icon and wordmark are fixed in `assets/templates/base.html` so
/// they are identical on every site; services only choose the brand link
/// target, the menu (usually [`site_menu`]) and the breadcrumbs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SiteHeader {
    /// Brand link target (usually `/`).
    pub brand_href: String,
    /// Left-aligned site menu entries (usually [`site_menu`]).
    pub menu: Vec<MenuItem>,
    /// Where the user is within this service — earlier segments are links.
    pub breadcrumbs: Vec<Breadcrumb>,
}
impl SiteHeader {
    /// Minimal header: fixed brand linking to `/`, no menu, no breadcrumbs.
    #[must_use]
    pub fn new() -> Self {
        Self {
            brand_href: "/".into(),
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

    /// Default header with the standard site menu and nothing highlighted.
    #[must_use]
    pub fn home() -> Self {
        Self::new().with_menu(site_menu(None))
    }
}
