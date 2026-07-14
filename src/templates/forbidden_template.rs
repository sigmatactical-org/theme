//! [`ForbiddenTemplate`].

#[allow(unused_imports)]
use super::*;
use crate::nav::SiteHeader;
use askama::Template;

#[derive(Template)]
#[template(path = "error/403.html")]
pub struct ForbiddenTemplate {
    pub site_header: SiteHeader,
    pub site_nav: String,
    pub copyright_years: String,
}
