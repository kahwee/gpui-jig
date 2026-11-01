//! Tabs component parts.
//!
//! Compose a root, list, tabs and the active panel. Keep the selected document
//! in your view and pass its state to each tab. IDs should identify documents,
//! not their current positions, so they remain stable when tabs are reordered.
//!
//! ```rust
//! use gpui::{prelude::*, *};
//! use gpui_jig::tabs;
//!
//! let selected = true;
//! let element = tabs::root("documents", "Documents")
//!     .child(tabs::list("document-tabs", "Open documents").child(
//!         tabs::tab("notes-tab", "Notes", selected)
//!             .px_2().py_1().child("Notes"),
//!     ))
//!     .child(tabs::panel("notes-panel", "Notes").child("Document content"));
//! ```
//!
//! This is the anatomy only: the application connects clicks and keyboard
//! input, changes the selected document, renders the matching panel and moves
//! focus. [`destination`] calculates horizontal arrow, Home and End targets;
//! it does not move focus or select a tab. See the [Cratebook tabs story](https://github.com/kahwee/cratebook/blob/main/src/stories/tabs.rs)
//! for an interactive example, including a separate close button.
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

/// Calculate a destination in the current horizontal tab order.
///
/// Left and right wrap; Home and End select the first and last index. Returns
/// `None` for other keys or an invalid `current`, including an empty list.
/// The caller owns selection, focus, and the mapping from indices to stable IDs.
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
