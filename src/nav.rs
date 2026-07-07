//! Service-provided configuration for the shared site navbar shell in
//! `assets/templates/base.html`.
//!
//! Each web service builds a [`SiteHeader`] for the current page (brand,
//! optional breadcrumb trail, optional menu label) and passes it with the
//! rendered identity-nav action widgets (`site_nav` in templates) when
//! extending the theme base layout.

/// One segment of the navbar breadcrumb trail (labels only — hrefs are not shown).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Breadcrumb {
    pub label: String,
    /// Link target; empty when this segment is the current page.
    pub href: String,
}

impl Breadcrumb {
    /// Navigable segment (earlier steps in the trail).
    #[must_use]
    pub fn link(href: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
        }
    }

    /// Current page — rendered as plain text, not a link.
    #[must_use]
    pub fn current(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: String::new(),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_sets_breadcrumbs() {
        let header = SiteHeader::new("Sigma Cart").with_breadcrumbs([
            Breadcrumb::link("http://store.example/", "Store"),
            Breadcrumb::current("Cart"),
        ]);
        assert_eq!(header.brand, "Sigma Cart");
        assert_eq!(header.breadcrumbs.len(), 2);
        assert_eq!(header.breadcrumbs[0].label, "Store");
        assert!(header.breadcrumbs[1].href.is_empty());
        assert_eq!(header.breadcrumbs[1].label, "Cart");
    }
}
