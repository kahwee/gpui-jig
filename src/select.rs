//! Select component parts.
pub use crate::collapsible::{root, trigger};
use gpui::{prelude::*, *};
pub fn value() -> Div {
    div()
}
pub fn popup() -> Div {
    div()
}
pub fn item_text() -> Div {
    div()
}
surface!(list, ListBox);
pub fn item(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    selected: bool,
) -> Stateful<Div> {
    crate::button::root(id, label)
        .role(Role::ListBoxOption)
        .aria_selected(selected)
}
