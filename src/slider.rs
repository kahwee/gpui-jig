//! Slider component parts.
use gpui::{prelude::*, *};
surface!(root, Group);
pub fn track() -> Div {
    div()
}
pub fn thumb(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    range: &crate::state::Range,
) -> Stateful<Div> {
    div()
        .id(id)
        .role(Role::Slider)
        .aria_label(label)
        .focusable()
        .tab_stop(true)
        .aria_min_numeric_value(range.min())
        .aria_max_numeric_value(range.max())
        .aria_numeric_value(range.value())
}
