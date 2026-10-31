//! Context menu component parts.
pub use crate::menu::{
    group, group_label, item, popup, root, separator, submenu_root, submenu_trigger,
};
use gpui::{prelude::*, *};
/// Wrap any target; the listener receives the pointer position in window coordinates.
pub fn trigger(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    open: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    div()
        .id(id)
        .aria_label(label)
        .on_mouse_down(MouseButton::Right, move |event, window, cx| {
            cx.stop_propagation();
            open(event, window, cx);
        })
}
/// Mount a styled popup above its target and fit it inside the window.
pub fn positioner(position: Point<Pixels>, popup: impl IntoElement) -> impl IntoElement {
    deferred(anchored().position(position).snap_to_window().child(popup))
}
