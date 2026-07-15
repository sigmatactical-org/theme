//! [`MenuItem`].

#[allow(unused_imports)]
use super::*;

/// One entry in the left-aligned site menu (e.g. `Store`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MenuItem {
    pub label: String,
    /// Link target (usually a service public base URL).
    pub href: String,
    /// Whether this entry is the site currently being viewed.
    pub active: bool,
}
impl MenuItem {
    /// Menu entry linking to another Sigma site.
    #[must_use]
    pub fn link(href: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
            active: false,
        }
    }

    /// Marks this entry as the site currently being viewed.
    #[must_use]
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}
