//! Shared HTTP security header values.

/// CSP fragment required by the self-hosted ALTCHA widget (PoW workers use `blob:` URLs).
pub const CSP_ALTCHA: &str = "worker-src 'self' blob: data:";

/// Baseline CSP for public HTML pages served by Sigma apps.
#[must_use]
pub fn public_html_csp(connect_src_extra: &str, style_unsafe_inline: bool) -> String {
    let style_src = if style_unsafe_inline {
        "style-src 'self' 'unsafe-inline'"
    } else {
        "style-src 'self'"
    };
    let connect = if connect_src_extra.is_empty() {
        "connect-src 'self'".to_string()
    } else {
        format!("connect-src 'self' {connect_src_extra}")
    };
    format!(
        "default-src 'self'; base-uri 'self'; object-src 'none'; frame-ancestors 'none'; \
         img-src 'self' data:; {style_src}; script-src 'self'; font-src 'self'; \
         {CSP_ALTCHA}; {connect}; form-action 'self'"
    )
}

/// Production CSP: same as [`public_html_csp`] plus upgrade-insecure-requests.
#[must_use]
pub fn public_html_csp_production(connect_src_extra: &str, style_unsafe_inline: bool) -> String {
    format!(
        "{}; upgrade-insecure-requests",
        public_html_csp(connect_src_extra, style_unsafe_inline)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_html_csp_allows_altcha_workers() {
        let csp = public_html_csp("", false);
        assert!(csp.contains("worker-src 'self' blob: data:"));
    }
}
