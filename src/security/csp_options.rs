//! [`CspOptions`] — the per-service knobs on the shared CSP.

/// Per-service CSP adjustments for [`crate::security::public_html_csp_with`].
///
/// The baseline is deliberately strict (`style-src 'self'`, `form-action
/// 'self'`); a service opts out only where it must. Prefer moving inline CSS
/// to a served asset over setting `style_unsafe_inline`.
///
/// Fields are owned so the warp/axum helpers that consume this can return
/// `'static` filters without borrowing a config string.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CspOptions {
    /// Appended to `connect-src` (e.g. the identity BFF origin). Empty means
    /// `'self'` only.
    pub connect_src_extra: String,
    /// Appended to `form-action` — needed when a page posts cross-origin
    /// (e.g. the storefront's add-to-cart POST to the cart service).
    pub form_action_extra: String,
    /// Allow inline `style` attributes and `<style>` blocks.
    pub style_unsafe_inline: bool,
    /// Append `upgrade-insecure-requests`.
    pub upgrade_insecure_requests: bool,
}

impl CspOptions {
    /// Production defaults: strict, with `upgrade-insecure-requests`.
    #[must_use]
    pub fn production(connect_src_extra: impl Into<String>) -> Self {
        Self {
            connect_src_extra: connect_src_extra.into(),
            upgrade_insecure_requests: true,
            ..Self::default()
        }
    }

    /// Allow cross-origin form posts to `origin`.
    #[must_use]
    pub fn form_action(mut self, origin: impl Into<String>) -> Self {
        self.form_action_extra = origin.into();
        self
    }

    /// Allow inline styles.
    #[must_use]
    pub fn style_unsafe_inline(mut self) -> Self {
        self.style_unsafe_inline = true;
        self
    }
}
