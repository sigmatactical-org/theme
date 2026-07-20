//! Shared HTTP security header values.

mod csp_options;

pub use csp_options::CspOptions;

/// CSP fragment required by the self-hosted ALTCHA widget (PoW workers use `blob:` URLs).
pub const CSP_ALTCHA: &str = "worker-src 'self' blob: data:";

/// Baseline CSP for public HTML pages served by Sigma apps.
#[must_use]
pub fn public_html_csp(connect_src_extra: &str, style_unsafe_inline: bool) -> String {
    public_html_csp_with(&CspOptions {
        connect_src_extra: connect_src_extra.to_owned(),
        style_unsafe_inline,
        ..CspOptions::default()
    })
}

/// Production CSP: same as [`public_html_csp`] plus upgrade-insecure-requests.
#[must_use]
pub fn public_html_csp_production(connect_src_extra: &str, style_unsafe_inline: bool) -> String {
    public_html_csp_with(&CspOptions {
        connect_src_extra: connect_src_extra.to_owned(),
        style_unsafe_inline,
        upgrade_insecure_requests: true,
        ..CspOptions::default()
    })
}

/// Build the shared CSP with per-service adjustments. See [`CspOptions`].
#[must_use]
pub fn public_html_csp_with(options: &CspOptions) -> String {
    let style_src = if options.style_unsafe_inline {
        "style-src 'self' 'unsafe-inline'"
    } else {
        "style-src 'self'"
    };
    let connect = directive("connect-src", &options.connect_src_extra);
    let form_action = directive("form-action", &options.form_action_extra);
    let csp = format!(
        "default-src 'self'; base-uri 'self'; object-src 'none'; frame-ancestors 'none'; \
         img-src 'self' data:; {style_src}; script-src 'self'; font-src 'self'; \
         {CSP_ALTCHA}; {connect}; {form_action}"
    );
    if options.upgrade_insecure_requests {
        format!("{csp}; upgrade-insecure-requests")
    } else {
        csp
    }
}

/// `"<name> 'self'"`, plus `extra` when non-empty.
fn directive(name: &str, extra: &str) -> String {
    if extra.is_empty() {
        format!("{name} 'self'")
    } else {
        format!("{name} 'self' {extra}")
    }
}

/// Non-CSP security headers applied to every response by the axum and warp
/// helpers ([`crate::axum::security_headers`] / [`crate::warp::security_headers`]).
///
/// The CSP itself is built per service via [`public_html_csp_production`]
/// because its `connect-src` varies (e.g. the identity BFF origin).
pub const SECURITY_HEADERS: &[(&str, &str)] = &[
    ("x-content-type-options", "nosniff"),
    ("x-frame-options", "DENY"),
    ("referrer-policy", "strict-origin-when-cross-origin"),
    ("cross-origin-opener-policy", "same-origin"),
    ("permissions-policy", "geolocation=(), microphone=(), camera=()"),
    (
        "strict-transport-security",
        "max-age=63072000; includeSubDomains; preload",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_html_csp_allows_altcha_workers() {
        let csp = public_html_csp("", false);
        assert!(csp.contains("worker-src 'self' blob: data:"));
    }

    #[test]
    fn csp_options_defaults_are_strict() {
        let csp = public_html_csp_with(&CspOptions::default());
        assert!(csp.contains("style-src 'self';"));
        assert!(csp.contains("form-action 'self'"));
        assert!(!csp.contains("unsafe-inline"));
        assert!(!csp.contains("upgrade-insecure-requests"));
    }

    #[test]
    fn csp_options_widen_form_action_and_styles() {
        let csp = public_html_csp_with(
            &CspOptions::production("http://identity.example")
                .form_action("http://cart.example")
                .style_unsafe_inline(),
        );
        assert!(csp.contains("form-action 'self' http://cart.example"));
        assert!(csp.contains("style-src 'self' 'unsafe-inline'"));
        assert!(csp.contains("connect-src 'self' http://identity.example"));
        assert!(csp.contains("upgrade-insecure-requests"));
    }

    #[test]
    fn production_csp_appends_extra_connect_src_and_upgrade() {
        let csp = public_html_csp_production("http://identity.example", false);
        assert!(csp.contains("connect-src 'self' http://identity.example"));
        assert!(csp.ends_with("upgrade-insecure-requests"));
        assert!(csp.contains("worker-src 'self' blob: data:"));
    }

    #[test]
    fn security_headers_cover_the_shared_set() {
        let names: Vec<&str> = SECURITY_HEADERS.iter().map(|(name, _)| *name).collect();
        for expected in [
            "x-content-type-options",
            "x-frame-options",
            "referrer-policy",
            "cross-origin-opener-policy",
            "permissions-policy",
            "strict-transport-security",
        ] {
            assert!(names.contains(&expected), "missing {expected}");
        }
    }
}
