//! Menu component parts.
pub use crate::collapsible::{root, trigger};
use gpui::{prelude::*, *};
surface!(popup, Menu);
action!(item, MenuItem);
surface!(group, Group);
surface!(group_label, Label);
pub fn separator() -> Div {
    div()
}
pub use root as submenu_root;
pub fn submenu_trigger(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    expanded: bool,
) -> Stateful<Div> {
    item(id, label).aria_expanded(expanded)
}

/// An independently toggleable command. Attach a listener to update the value.
pub fn checkbox_item(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    checked: bool,
) -> Stateful<Div> {
    item(id, label)
        .role(Role::MenuItemCheckBox)
        .aria_toggled(checked.into())
}
// Group mutually exclusive commands. The caller owns the selected value.
surface!(radio_group, RadioGroup);
/// One choice in a menu radio group.
pub fn radio_item(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    selected: bool,
) -> Stateful<Div> {
    item(id, label)
        .role(Role::MenuItemRadio)
        .aria_toggled(selected.into())
}
