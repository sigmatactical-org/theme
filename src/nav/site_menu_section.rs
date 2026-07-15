//! [`SiteMenuSection`].

#[allow(unused_imports)]
use super::*;

/// Which standard menu entry the current site is, for highlighting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SiteMenuSection {
    Store,
    Orders,
    Updates,
}
