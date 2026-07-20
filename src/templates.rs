mod forbidden_template;
mod internal_error_template;
mod method_not_allowed_template;
mod not_found_template;
pub use forbidden_template::ForbiddenTemplate;
pub use internal_error_template::InternalErrorTemplate;
pub use method_not_allowed_template::MethodNotAllowedTemplate;
pub use not_found_template::NotFoundTemplate;

use askama::Template;

use crate::copyright_years;
use crate::nav::SiteHeader;

/// Builds an error-page template with the default header/nav/footer fields
/// (all four error templates share the same shape) and renders it.
fn render_error_page<T: Template>(
    template: impl FnOnce(SiteHeader, String, String) -> T,
) -> Result<String, askama::Error> {
    template(SiteHeader::home(), String::new(), copyright_years()).render()
}

/// Renders the 404 page HTML.
///
/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_not_found_html() -> Result<String, askama::Error> {
    render_error_page(|site_header, site_nav, copyright_years| NotFoundTemplate {
        site_header,
        site_nav,
        copyright_years,
    })
}

/// Renders the 500 page HTML.
///
/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_internal_server_error_html() -> Result<String, askama::Error> {
    render_error_page(|site_header, site_nav, copyright_years| InternalErrorTemplate {
        site_header,
        site_nav,
        copyright_years,
    })
}

/// Renders the 403 page HTML.
///
/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_forbidden_html() -> Result<String, askama::Error> {
    render_error_page(|site_header, site_nav, copyright_years| ForbiddenTemplate {
        site_header,
        site_nav,
        copyright_years,
    })
}

/// Renders the 405 page HTML.
///
/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_method_not_allowed_html() -> Result<String, askama::Error> {
    render_error_page(
        |site_header, site_nav, copyright_years| MethodNotAllowedTemplate {
            site_header,
            site_nav,
            copyright_years,
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_templates_render() {
        let html = render_not_found_html().expect("404 template");
        assert!(html.contains("Oops"));
        let html = render_internal_server_error_html().expect("500 template");
        assert!(html.contains("Something went wrong"));
        let html = render_forbidden_html().expect("403 template");
        assert!(html.contains("Access denied"));
        let html = render_method_not_allowed_html().expect("405 template");
        assert!(html.contains("Method not allowed"));
    }

    #[test]
    fn internal_error_template_renders_html() {
        let html = render_internal_server_error_html().expect("500 template");
        assert!(html.contains("Something went wrong"));
        assert!(html.contains("Oops"));
        assert!(html.contains("<title>Something went wrong — Sigma Tactical Group</title>"));
    }
}
