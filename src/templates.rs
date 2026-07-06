use askama::Template;

use crate::copyright_years;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub copyright_years: String,
}

#[derive(Template)]
#[template(path = "error/404.html")]
pub struct NotFoundTemplate {
    pub copyright_years: String,
}

#[derive(Template)]
#[template(path = "error/500.html")]
pub struct InternalErrorTemplate {
    pub copyright_years: String,
}

#[derive(Template)]
#[template(path = "error/403.html")]
pub struct ForbiddenTemplate {
    pub copyright_years: String,
}

#[derive(Template)]
#[template(path = "error/405.html")]
pub struct MethodNotAllowedTemplate {
    pub copyright_years: String,
}

/// Renders the home page HTML.
///
/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_index_html() -> Result<String, askama::Error> {
    IndexTemplate {
        copyright_years: copyright_years(),
    }
    .render()
}

/// Renders the 404 page HTML.
///
/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_not_found_html() -> Result<String, askama::Error> {
    NotFoundTemplate {
        copyright_years: copyright_years(),
    }
    .render()
}

/// Renders the 500 page HTML.
///
/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_internal_server_error_html() -> Result<String, askama::Error> {
    InternalErrorTemplate {
        copyright_years: copyright_years(),
    }
    .render()
}

/// Renders the 403 page HTML.
///
/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_forbidden_html() -> Result<String, askama::Error> {
    ForbiddenTemplate {
        copyright_years: copyright_years(),
    }
    .render()
}

/// Renders the 405 page HTML.
///
/// # Errors
///
/// Returns [`askama::Error`] when template rendering fails.
pub fn render_method_not_allowed_html() -> Result<String, askama::Error> {
    MethodNotAllowedTemplate {
        copyright_years: copyright_years(),
    }
    .render()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::copyright_years;

    #[test]
    fn rendered_html_contains_title_and_assets() {
        let html = render_index_html().expect("template should render");
        assert!(html.contains("<title>Sigma Tactical Group</title>"));
        assert!(html.contains("sigma-dial-root"));
        assert!(html.contains("/static/js/sigma-dial.js"));
        assert!(html.contains("/static/css/sigma-dial.css"));
        assert!(html.contains(&format!(
            "&copy; {} Sigma Tactical Group. All rights reserved.",
            copyright_years()
        )));
    }

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
}
