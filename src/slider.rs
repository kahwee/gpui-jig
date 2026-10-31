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

/// Apply a conventional slider key to a bounded value. Page keys move ten
/// steps. Returns false for unrelated keys and invalid step sizes so callers
/// can leave those events available to the surrounding workspace.
///
/// ```
/// use gpui_jig::{slider::apply_key, state::Range};
/// let mut zoom = Range::new(50., 200., 100.).unwrap();
/// assert!(apply_key(&mut zoom, "pageup", 10.));
/// assert_eq!(zoom.value(), 200.);
/// assert!(apply_key(&mut zoom, "home", 10.));
/// assert_eq!(zoom.value(), 50.);
/// assert!(!apply_key(&mut zoom, "tab", 10.));
/// ```
pub fn apply_key(range: &mut crate::state::Range, key: &str, step: f64) -> bool {
    if !step.is_finite() || step <= 0. {
        return false;
    }
    match key {
        "left" | "down" => range.step(-step),
        "right" | "up" => range.step(step),
        "pagedown" => range.step(-10. * step),
        "pageup" => range.step(10. * step),
        "home" => range.set(range.min()),
        "end" => range.set(range.max()),
        _ => return false,
    }
    true
}
