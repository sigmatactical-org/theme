//! [`site_menu`].

#[allow(unused_imports)]
use super::*;

fn service_url(var: &str, default: &str) -> String {
    let mut url = std::env::var(var)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .map_or_else(|| default.to_string(), |value| value.trim().to_string());
    if !url.ends_with('/') {
        url.push('/');
    }
    url
}

/// Standard cross-site menu shown left-aligned in the navbar on every Sigma
/// site: Store, Orders, Updates.
///
/// Link targets come from `SIGMA_STORE_PUBLIC_URL`, `SIGMA_ORDERS_PUBLIC_URL`
/// and `SIGMA_UPDATES_PUBLIC_URL`, falling back to the local development
/// ports. `active` highlights the entry for the site being viewed.
#[must_use]
pub fn site_menu(active: Option<SiteMenuSection>) -> Vec<MenuItem> {
    vec![
        MenuItem::link(
            service_url("SIGMA_STORE_PUBLIC_URL", "http://127.0.0.1:8082/"),
            "Store",
        )
        .with_active(active == Some(SiteMenuSection::Store)),
        MenuItem::link(
            service_url("SIGMA_ORDERS_PUBLIC_URL", "http://127.0.0.1:8085/"),
            "Orders",
        )
        .with_active(active == Some(SiteMenuSection::Orders)),
        MenuItem::link(
            service_url("SIGMA_UPDATES_PUBLIC_URL", "http://127.0.0.1:8080/"),
            "Updates",
        )
        .with_active(active == Some(SiteMenuSection::Updates)),
    ]
}
