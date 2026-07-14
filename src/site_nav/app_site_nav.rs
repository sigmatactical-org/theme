//! [`AppSiteNav`].

#[allow(unused_imports)]
use super::*;

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
