//! Number field component parts.
use gpui::{prelude::*, *};
surface!(root, Group);
pub use crate::button::root as increment;
pub use crate::button::root as decrement;
pub fn input(
    editor: Stateful<Div>,
    label: impl Into<SharedString>,
    range: &crate::state::Range,
) -> Stateful<Div> {
    editor
        .role(Role::SpinButton)
        .aria_label(label)
        .aria_min_numeric_value(range.min())
        .aria_max_numeric_value(range.max())
        .aria_numeric_value(range.value())
}
