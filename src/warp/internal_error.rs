//! [`InternalError`].

/// Rejection marker for a failure that is this service's fault -- a query that
/// errored, a template that would not render -- recovered to the themed 500
/// page by [`super::handle_rejection`].
///
/// Construct it through [`super::internal_rejection`], which also records the
/// cause.
#[derive(Debug)]
pub struct InternalError;

impl warp::reject::Reject for InternalError {}
