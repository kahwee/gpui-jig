use gpui::StatefulInteractiveElement;
use gpui::{Div, ElementId, SharedString, Stateful, div};

/// Groups the trigger and its optional panel.
pub fn root() -> Div {
    div()
}

/// The owner renders the panel from the same expanded state passed to its trigger.
/// Attach the state change with `on_click`, as for an ordinary button.
pub fn trigger(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    expanded: bool,
) -> Stateful<Div> {
    crate::button::root(id, label).aria_expanded(expanded)
}

/// Content surface for an expanded collapsible.
pub fn panel() -> Div {
    div()
}
