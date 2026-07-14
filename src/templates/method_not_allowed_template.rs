//! [`MethodNotAllowedTemplate`].

#[allow(unused_imports)]
use super::*;
use crate::nav::SiteHeader;
use askama::Template;

#[derive(Template)]
#[template(path = "error/405.html")]
pub struct MethodNotAllowedTemplate {
    pub site_header: SiteHeader,
    pub site_nav: String,
    pub copyright_years: String,
}
