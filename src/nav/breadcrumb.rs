//! [`Breadcrumb`].

#[allow(unused_imports)]
use super::*;

/// One segment of the navbar breadcrumb trail (labels only — hrefs are not shown).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Breadcrumb {
    pub label: String,
    /// Link target; empty when this segment is the current page.
    pub href: String,
}
impl Breadcrumb {
    /// Navigable segment (earlier steps in the trail).
    #[must_use]
    pub fn link(href: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
        }
    }

    /// Current page — rendered as plain text, not a link.
    #[must_use]
    pub fn current(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: String::new(),
        }
    }
}
