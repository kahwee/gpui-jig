//! Tabs component parts.
use gpui::{prelude::*, *};
surface!(root, Group);
surface!(list, TabList);
surface!(panel, TabPanel);
pub fn tab(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    selected: bool,
) -> Stateful<Div> {
    crate::button::root(id, label)
        .role(Role::Tab)
        .aria_selected(selected)
        .tab_stop(selected)
}

/// Move within the visible horizontal tab list. Return an index into the
/// caller's current order, so stable document IDs can survive a close.
pub fn destination(key: &str, current: usize, count: usize) -> Option<usize> {
    if current >= count {
        return None;
    }
    match key {
        "left" => Some(current.checked_sub(1).unwrap_or(count - 1)),
        "right" => Some(if current + 1 == count { 0 } else { current + 1 }),
        "home" => Some(0),
        "end" => Some(count - 1),
        _ => None,
    }
}

/// Render beside the tab trigger so closing a document is a separate action.
pub use crate::button::root as close;
