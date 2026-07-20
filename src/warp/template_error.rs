//! [`TemplateError`].

/// Rejection marker for template render failures; recovered to the themed
/// 500 page by [`super::handle_rejection`].
#[derive(Debug)]
pub struct TemplateError;

impl warp::reject::Reject for TemplateError {}
