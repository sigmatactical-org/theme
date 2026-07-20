//! [`ThemedPanicResponse`].

use std::any::Any;

use axum::body::Body;
use axum::response::Response;
use tower_http::catch_panic::ResponseForPanic;

use super::internal_server_error_response;

/// Panic handler that returns the themed 500 page instead of an empty response.
#[derive(Clone, Copy, Debug)]
pub struct ThemedPanicResponse;

impl ResponseForPanic for ThemedPanicResponse {
    type ResponseBody = Body;

    fn response_for_panic(
        &mut self,
        _err: Box<dyn Any + Send + 'static>,
    ) -> Response<Self::ResponseBody> {
        internal_server_error_response()
    }
}
