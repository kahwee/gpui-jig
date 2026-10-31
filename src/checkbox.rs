//! Checkbox component parts.
//!
//! Use the same checked value for the semantic state and the visible indicator.
//!
//! ```rust
//! use gpui::{prelude::*, *};
//! use gpui_jig::checkbox;
//! let checked = true;
//! let element = checkbox::root("updates", "Receive updates", checked)
//!     .when(checked, |s| s.child(checkbox::indicator().child("✓")))
//!     .child("Receive updates");
//! ```
use gpui::{prelude::*, *};
checkable!(CheckBox, indicator);

/// A controlled checkbox that can announce a partially selected group.
/// Use the same value to draw the indicator (check, dash, or empty).
pub fn tri_state(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    state: Toggled,
) -> Stateful<Div> {
    crate::button::root(id, label)
        .role(Role::CheckBox)
        .aria_toggled(state)
}
