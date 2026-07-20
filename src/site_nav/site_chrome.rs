//! [`SiteChrome`].

use crate::nav::{SiteHeader, SiteMenuSection, page_header};

use super::{AppSiteNav, render_app_site_nav};

/// Per-service inputs for the standard site chrome: the area wordmark shown
/// in the page header plus the base URLs the header action widgets link
/// against. Build one from the service's config at startup (or per request)
/// and derive the header and nav for each page from it.
#[derive(Clone, Debug)]
pub struct SiteChrome {
    /// Area wordmark next to the sigma symbol (e.g. `Store`).
    pub title: String,
    /// Public base URL of the identity BFF.
    pub identity_base: String,
    /// Public base URL of this service (used as the auth/contact return base).
    pub app_base: String,
    /// Public base URL of the contact service.
    pub contact_base: String,
    /// Cart page URL for the header cart icon.
    pub cart_url: String,
    /// Whether the header shows the cart icon.
    pub show_cart: bool,
}

impl SiteChrome {
    /// Standard page header for this area: wordmark plus the shared site
    /// menu, with `active` highlighting the site being viewed.
    #[must_use]
    pub fn page_header(&self, active: Option<SiteMenuSection>) -> SiteHeader {
        page_header(self.title.clone(), active)
    }

    /// Render the standard header actions (sign-in / welcome widget and the
    /// optional cart icon) for the page at `return_path`.
    ///
    /// # Errors
    ///
    /// Returns [`askama::Error`] when template rendering fails.
    pub fn site_nav(&self, return_path: &str, cart_count: u32) -> Result<String, askama::Error> {
        render_app_site_nav(&AppSiteNav {
            identity_base: &self.identity_base,
            app_base: &self.app_base,
            contact_base: &self.contact_base,
            cart_url: &self.cart_url,
            cart_count,
            return_path,
            show_cart: self.show_cart,
            show_contact_us: false,
            leading_html: "",
        })
    }
}
