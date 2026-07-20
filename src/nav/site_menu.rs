//! [`site_menu`].

use std::sync::OnceLock;

use super::{MenuItem, SiteMenuSection};

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

/// Store / Orders / Updates link targets, resolved from the environment once
/// per process and cached (services set the `SIGMA_*_PUBLIC_URL` variables at
/// startup and never change them afterwards).
fn service_urls() -> &'static [String; 3] {
    static URLS: OnceLock<[String; 3]> = OnceLock::new();
    URLS.get_or_init(|| {
        [
            service_url("SIGMA_STORE_PUBLIC_URL", "http://127.0.0.1:8082/"),
            service_url("SIGMA_ORDERS_PUBLIC_URL", "http://127.0.0.1:8085/"),
            service_url("SIGMA_UPDATES_PUBLIC_URL", "http://127.0.0.1:8080/"),
        ]
    })
}

/// Standard cross-site menu shown left-aligned in the navbar on every Sigma
/// site: Store, Orders, Updates.
///
/// Link targets come from `SIGMA_STORE_PUBLIC_URL`, `SIGMA_ORDERS_PUBLIC_URL`
/// and `SIGMA_UPDATES_PUBLIC_URL`, falling back to the local development
/// ports. `active` highlights the entry for the site being viewed.
#[must_use]
pub fn site_menu(active: Option<SiteMenuSection>) -> Vec<MenuItem> {
    let [store, orders, updates] = service_urls();
    vec![
        MenuItem::link(store.clone(), "Store").with_active(active == Some(SiteMenuSection::Store)),
        MenuItem::link(orders.clone(), "Orders")
            .with_active(active == Some(SiteMenuSection::Orders)),
        MenuItem::link(updates.clone(), "Updates")
            .with_active(active == Some(SiteMenuSection::Updates)),
    ]
}
