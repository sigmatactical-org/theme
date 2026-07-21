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

/// Cross-site link targets, resolved from the environment once per process and
/// cached (services set the `SIGMA_*_PUBLIC_URL` variables at startup and never
/// change them afterwards).
fn service_urls() -> &'static [String; 8] {
    static URLS: OnceLock<[String; 8]> = OnceLock::new();
    URLS.get_or_init(|| {
        [
            service_url("SIGMA_STORE_PUBLIC_URL", "http://127.0.0.1:8082/"),
            service_url("SIGMA_ORDERS_PUBLIC_URL", "http://127.0.0.1:8085/"),
            service_url("SIGMA_UPDATES_PUBLIC_URL", "http://127.0.0.1:8080/"),
            service_url("SIGMA_SIGMATACTICAL_ORG_PUBLIC_URL", "http://127.0.0.1:8080/"),
            service_url("SIGMA_IDENTITY_PUBLIC_URL", "http://127.0.0.1:3000/"),
            service_url("SIGMA_SENTRY_PUBLIC_URL", "http://127.0.0.1:8080/"),
            service_url("SIGMA_CONTACT_PUBLIC_URL", "http://127.0.0.1:8083/"),
            service_url("SIGMA_SIGMATACTICALGROUP_COM_PUBLIC_URL", "http://127.0.0.1:8080/"),
        ]
    })
}

/// Standard cross-site menu shown left-aligned in the navbar on every Sigma
/// site: Store, Orders, a Resources dropdown and an About dropdown.
///
/// Link targets come from the `SIGMA_*_PUBLIC_URL` variables, falling back to
/// the local development ports. Resources groups Developer Zone
/// (`sigmatactical.org`), Account (`identity`) and Updates; About groups
/// Contact Us (`contact`), Corporate Affairs (`sigmatacticalgroup.com`) and
/// Sentry. `active` highlights the top-level entry for the site being viewed.
#[must_use]
pub fn site_menu(active: Option<SiteMenuSection>) -> Vec<MenuItem> {
    let [store, orders, updates, sigmatactical_org, identity, sentry, contact, sigmatacticalgroup_com] =
        service_urls();
    vec![
        MenuItem::link(store.clone(), "Store").with_active(active == Some(SiteMenuSection::Store)),
        MenuItem::link(orders.clone(), "Orders")
            .with_active(active == Some(SiteMenuSection::Orders)),
        MenuItem::dropdown(
            "Resources",
            [
                MenuItem::link(sigmatactical_org.clone(), "Developer Zone"),
                MenuItem::link(identity.clone(), "Account"),
                MenuItem::link(updates.clone(), "Updates"),
            ],
        ),
        MenuItem::dropdown(
            "About",
            [
                MenuItem::link(contact.clone(), "Contact Us"),
                MenuItem::link(sigmatacticalgroup_com.clone(), "Corporate Affairs"),
                MenuItem::link(sentry.clone(), "Sentry"),
            ],
        ),
    ]
}
