//! Service-provided configuration for the shared site navbar shell in
//! `assets/templates/base.html`.
//!
//! Each web service builds a [`SiteHeader`] for the current page (brand,
//! optional breadcrumb trail, optional menu label) and passes it with the
//! rendered site action widgets ([`crate::site_nav`]) when extending the theme
//! base layout.

mod breadcrumb;
mod site_header;
pub use breadcrumb::Breadcrumb;
pub use site_header::SiteHeader;

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
