use gpui::{Div, ElementId, Role, SharedString, Stateful, div, prelude::*};

/// A focusable action. Attach a single `on_click` listener: GPUI dispatches
/// pointer, accessibility and focused Enter/Space activation through that path.
pub fn root(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    div()
        .id(id)
        .role(Role::Button)
        .aria_label(label)
        .focusable()
        .tab_stop(true)
}
