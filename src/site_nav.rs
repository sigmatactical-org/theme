//! Compose header action widgets (sign-in, cart, contact) for [`crate::nav::SiteHeader`].
//!
//! Each nav crate owns one affordance; this module stitches them together for
//! `assets/templates/base.html`.

use askama::Template;
use sigma_cart_nav::render_cart_nav;
use sigma_contact_nav::{contact_us_url, render_contact_nav};
use sigma_identity_nav::{AuthLinks, auth_links, render_auth_nav};

#[derive(Template)]
#[template(path = "widgets/site_nav.html")]
struct SiteNavTemplate<'a> {
    leading_html: &'a str,
    auth_nav: &'a str,
    cart_nav: &'a str,
    contact_nav: &'a str,
}

/// Render the standard Sigma header actions: optional leading link, sign-in /
/// welcome widget, optional cart icon, and optionally a contact-us button.
///
/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_site_nav(
    links: &AuthLinks,
    contact_us_url: &str,
    cart_url: &str,
    cart_count: u32,
    show_cart: bool,
    show_contact_us: bool,
    leading_html: &str,
) -> Result<String, askama::Error> {
    let auth_nav = render_auth_nav(links)?;
    let cart_nav = if show_cart {
        render_cart_nav(cart_url, cart_count)?
    } else {
        String::new()
    };
    let contact_nav = if show_contact_us {
        render_contact_nav(contact_us_url)?
    } else {
        String::new()
    };
    SiteNavTemplate {
        leading_html,
        auth_nav: &auth_nav,
        cart_nav: &cart_nav,
        contact_nav: &contact_nav,
    }
    .render()
}

/// Inputs for [`render_app_site_nav`].
pub struct AppSiteNav<'a> {
    pub identity_base: &'a str,
    pub app_base: &'a str,
    pub contact_base: &'a str,
    pub cart_url: &'a str,
    pub cart_count: u32,
    pub return_path: &'a str,
    pub show_cart: bool,
    pub show_contact_us: bool,
    pub leading_html: &'a str,
}

/// Convenience wrapper that builds per-service URLs and renders the header nav.
///
/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_app_site_nav(input: &AppSiteNav<'_>) -> Result<String, askama::Error> {
    let links = auth_links(input.identity_base, input.app_base, input.return_path);
    let contact_url = contact_us_url(input.contact_base, input.app_base, input.return_path);
    render_site_nav(
        &links,
        &contact_url,
        input.cart_url,
        input.cart_count,
        input.show_cart,
        input.show_contact_us,
        input.leading_html,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn site_nav_includes_auth_cart_and_contact() {
        let links = auth_links("http://identity.example", "http://store.example", "/");
        let contact = contact_us_url(
            "http://contact.example",
            "http://store.example",
            "/",
        );
        let html =
            render_site_nav(&links, &contact, "http://cart.example/", 2, true, true, "")
                .expect("render");
        assert!(html.contains("store-nav-auth"));
        assert!(html.contains("href=\"http://cart.example/\""));
        assert!(html.contains("Contact us"));
        assert!(html.contains(">2</span>"));
    }

    #[test]
    fn site_nav_can_hide_contact_us() {
        let links = auth_links("http://identity.example", "http://contact.example", "/contact");
        let contact = contact_us_url(
            "http://contact.example",
            "http://contact.example",
            "/contact",
        );
        let html =
            render_site_nav(&links, &contact, "http://cart.example/", 0, true, false, "")
                .expect("render");
        assert!(html.contains("store-nav-auth"));
        assert!(!html.contains("Contact us"));
    }

    #[test]
    fn site_nav_can_hide_cart() {
        let links = auth_links("http://identity.example", "http://store.example", "/");
        let contact = contact_us_url(
            "http://contact.example",
            "http://store.example",
            "/",
        );
        let html =
            render_site_nav(&links, &contact, "http://cart.example/", 2, false, true, "")
                .expect("render");
        assert!(html.contains("store-nav-auth"));
        assert!(!html.contains("href=\"http://cart.example/\""));
        assert!(!html.contains(">2</span>"));
    }
}
