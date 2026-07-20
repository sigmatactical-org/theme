//! [`InternalErrorTemplate`].

use crate::nav::SiteHeader;
use askama::Template;

#[derive(Template)]
#[template(path = "error/500.html")]
pub struct InternalErrorTemplate {
    pub site_header: SiteHeader,
    pub site_nav: String,
    pub copyright_years: String,
}
