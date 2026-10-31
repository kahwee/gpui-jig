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
