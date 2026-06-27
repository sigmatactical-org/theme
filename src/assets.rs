use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/static/"]
pub(crate) struct StaticAssets;

#[must_use]
pub(crate) fn cache_control(path: &str) -> &'static str {
    if path.starts_with("vendor/") || path.starts_with("fonts/") {
        "public, max-age=31536000, immutable"
    } else {
        "public, max-age=3600"
    }
}

#[must_use]
pub(crate) fn content_type(path: &str) -> &'static str {
    mime_guess::from_path(path)
        .first_raw()
        .unwrap_or("application/octet-stream")
}
