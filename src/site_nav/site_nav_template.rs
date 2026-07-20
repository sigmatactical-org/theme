//! [`SiteNavTemplate`].

use askama::Template;

#[derive(Template)]
#[template(path = "widgets/site_nav.html")]
pub(crate) struct SiteNavTemplate<'a> {
    pub(crate) leading_html: &'a str,
    pub(crate) auth_nav: &'a str,
    pub(crate) cart_nav: &'a str,
    pub(crate) contact_nav: &'a str,
}
