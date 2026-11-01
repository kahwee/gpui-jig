//! Controlled, unstyled month grid. Keep one DateGrid per widget across renders.
//! Render Month::weeks as rows, using day for populated cells. Pass the same
//! selected date and first weekday to root and Month. The caller owns labels,
//! event markers, selection and styling.
mod month;
use chrono::Datelike;
use gpui::{prelude::*, *};
pub use month::{Month, NaiveDate, Weekday, navigate};
use std::rc::Rc;

pub struct DateGrid {
    handles: Rc<[FocusHandle; 31]>,
}
impl DateGrid {
    pub fn new(cx: &App) -> Self {
        Self {
            handles: Rc::new(std::array::from_fn(|_| cx.focus_handle())),
        }
    }
    pub fn root(
        &self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        selected: NaiveDate,
        start: Weekday,
        select: impl Fn(&NaiveDate, &mut Window, &mut App) + 'static,
    ) -> Stateful<Div> {
        let handles = self.handles.clone();
        div().id(id).role(Role::Grid).aria_label(label).on_key_down(
            move |event: &KeyDownEvent, w, cx| {
                if event.keystroke.modifiers.modified() {
                    return;
                }
                if let Some(next) = navigate(selected, start, &event.keystroke.key) {
                    w.prevent_default();
                    cx.stop_propagation();
                    w.focus(&handles[next.day0() as usize], cx);
                    select(&next, w, cx);
                }
            },
        )
    }
    pub fn day(
        &self,
        date: NaiveDate,
        selected: NaiveDate,
        label: impl Into<SharedString>,
        select: impl Fn(&NaiveDate, &mut Window, &mut App) + 'static,
    ) -> Stateful<Div> {
        let handle = self.handles[date.day0() as usize].clone();
        crate::button::root(("date", date.day0() as usize), label)
            .role(Role::GridCell)
            .track_focus(&handle)
            .tab_stop(date == selected)
            .aria_selected(date == selected)
            .on_click(move |_, w, cx| {
                w.focus(&handle, cx);
                select(&date, w, cx);
            })
    }
}
pub fn row(id: impl Into<ElementId>) -> Stateful<Div> {
    div().id(id).role(Role::Row)
}
