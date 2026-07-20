//! Service-provided configuration for the shared site navbar shell in
//! `assets/templates/base.html`.
//!
//! The navbar shows the fixed Sigma Tactical Group brand followed by the
//! left-aligned site menu ([`site_menu`]: Store, Orders, Updates), with the
//! rendered site action widgets ([`crate::site_nav`]) on the right. The
//! breadcrumb trail is rendered in its own bar under the navbar. Each web
//! service builds a [`SiteHeader`] for the current page and passes it when
//! extending the theme base layout.

mod breadcrumb;
mod menu_item;
mod nav_entry;
mod site_header;
mod site_menu;
mod site_menu_section;
pub use breadcrumb::Breadcrumb;
pub use menu_item::MenuItem;
pub use nav_entry::NavEntry;
pub use site_header::SiteHeader;
pub use site_menu::site_menu;
pub use site_menu_section::SiteMenuSection;

/// Standard page header for a service area: the area wordmark next to the
/// sigma symbol plus the shared cross-site menu, with `active` highlighting
/// the entry for the site being viewed.
#[must_use]
pub fn page_header(area: impl Into<String>, active: Option<SiteMenuSection>) -> SiteHeader {
    SiteHeader::new(area).with_menu(site_menu(active))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_sets_breadcrumbs() {
        let header = SiteHeader::new("Cart").with_breadcrumbs([
            Breadcrumb::link("http://store.example/", "Store"),
            Breadcrumb::current("Cart"),
        ]);
        assert_eq!(header.brand_href, "/");
        assert_eq!(header.breadcrumbs.len(), 2);
        assert_eq!(header.breadcrumbs[0].label, "Store");
        assert!(header.breadcrumbs[1].href.is_empty());
        assert_eq!(header.breadcrumbs[1].label, "Cart");
    }

    #[test]
    fn site_menu_lists_store_orders_updates_and_highlights_active() {
        let menu = site_menu(Some(SiteMenuSection::Orders));
        let labels: Vec<&str> = menu.iter().map(|item| item.label.as_str()).collect();
        assert_eq!(labels, ["Store", "Orders", "Updates"]);
        let active: Vec<bool> = menu.iter().map(|item| item.active).collect();
        assert_eq!(active, [false, true, false]);
        assert!(menu.iter().all(|item| item.href.ends_with('/')));
    }

    #[test]
    fn page_header_names_the_area_and_highlights_the_section() {
        let header = page_header("Orders", Some(SiteMenuSection::Orders));
        assert_eq!(header.brand_label, "Orders");
        assert_eq!(header.menu.len(), 3);
        assert!(header.menu[1].active);
    }
}
