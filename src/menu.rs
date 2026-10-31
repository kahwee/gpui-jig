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
