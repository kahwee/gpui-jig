//! Combobox component parts.
pub use crate::select::{item, list, root, trigger};
use gpui::{prelude::*, *};
pub fn input(editor: Stateful<Div>, label: impl Into<SharedString>, open: bool) -> Stateful<Div> {
    editor
        .role(Role::EditableComboBox)
        .aria_label(label)
        .aria_expanded(open)
}
