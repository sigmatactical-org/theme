//! [`MenuItem`].

/// One entry in the left-aligned site menu (e.g. `Store`).
///
/// A plain entry is a link to `href`. An entry with non-empty `children` is a
/// dropdown: its `label` toggles a submenu of the child entries and its own
/// `href` is unused.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MenuItem {
    pub label: String,
    /// Link target (usually a service public base URL). Empty for a dropdown.
    pub href: String,
    /// Whether this entry is the site currently being viewed.
    pub active: bool,
    /// Submenu entries; non-empty makes this a dropdown rather than a link.
    pub children: Vec<MenuItem>,
}
impl MenuItem {
    /// Menu entry linking to another Sigma site.
    #[must_use]
    pub fn link(href: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: href.into(),
            active: false,
            children: Vec::new(),
        }
    }

    /// Dropdown entry: `label` toggles a submenu of `children`.
    #[must_use]
    pub fn dropdown(
        label: impl Into<String>,
        children: impl IntoIterator<Item = MenuItem>,
    ) -> Self {
        Self {
            label: label.into(),
            href: String::new(),
            active: false,
            children: children.into_iter().collect(),
        }
    }

    /// Marks this entry as the site currently being viewed.
    #[must_use]
    pub fn with_active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}
