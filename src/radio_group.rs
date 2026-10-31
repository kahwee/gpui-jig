use gpui::{
    App, Div, ElementId, FocusHandle, KeyDownEvent, Role, SharedString, Stateful, Window, div,
    prelude::*,
};
use std::rc::Rc;

/// A fixed set of mutually exclusive choices. Keep this object across renders
/// for stable focus. The owner supplies the checked item and commits changes.
pub struct RadioGroup<const N: usize> {
    handles: Rc<[FocusHandle; N]>,
}

impl<const N: usize> RadioGroup<N> {
    pub fn new(cx: &App) -> Self {
        assert!(N > 0, "a radio group requires at least one choice");
        Self {
            handles: Rc::new(std::array::from_fn(|_| cx.focus_handle())),
        }
    }

    pub fn root(&self, id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
        div().id(id).role(Role::RadioGroup).aria_label(label)
    }

    pub fn focus_handle(&self, index: usize) -> &FocusHandle {
        &self.handles[index]
    }

    pub fn item(
        &self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        index: usize,
        checked: bool,
        select: impl Fn(&usize, &mut Window, &mut App) + 'static,
    ) -> Stateful<Div> {
        let select = Rc::new(select);
        let click = select.clone();
        let handles = self.handles.clone();
        let handle = handles[index].clone();
        div()
            .id(id)
            .role(Role::RadioButton)
            .aria_label(label)
            .aria_toggled(checked.into())
            .track_focus(&handle)
            .tab_stop(checked)
            .on_click(move |_, window, cx| {
                window.focus(&handle, cx);
                click(&index, window, cx);
            })
            .on_key_down(move |event: &KeyDownEvent, window, cx| {
                if event.keystroke.modifiers.modified() {
                    return;
                }
                let next = match event.keystroke.key.as_str() {
                    "left" | "up" => (index + N - 1) % N,
                    "right" | "down" => (index + 1) % N,
                    _ => return,
                };
                window.prevent_default();
                cx.stop_propagation();
                window.focus(&handles[next], cx);
                select(&next, window, cx);
            })
    }
}
