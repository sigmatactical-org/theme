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
mod site_header;
mod site_menu;
mod site_menu_section;
pub use breadcrumb::Breadcrumb;
pub use menu_item::MenuItem;
pub use site_header::SiteHeader;
pub use site_menu::site_menu;
pub use site_menu_section::SiteMenuSection;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_sets_breadcrumbs() {
        let header = SiteHeader::new().with_breadcrumbs([
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
}
