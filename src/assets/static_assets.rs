//! [`StaticAssets`].

use rust_embed::Embed;

/// Static theme assets embedded into the binary from `assets/static/`.
#[derive(Embed)]
#[folder = "assets/static/"]
pub(crate) struct StaticAssets;
