//! Controlled GPUI element constructors. IDs must be stable within the parent.
use gpui::{div, prelude::*, *};

macro_rules! surface {
    ($name:ident, $role:ident) => {
        pub fn $name(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
            div().id(id).role(Role::$role).aria_label(label)
        }
    };
}
macro_rules! action {
    ($name:ident, $role:ident) => {
        pub fn $name(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
            crate::button::root(id, label).role(Role::$role)
        }
    };
}
macro_rules! checkable {
    ($module:ident, $role:ident, $part:ident) => {
        pub mod $module {
            use super::*;
            pub fn root(
                id: impl Into<ElementId>,
                label: impl Into<SharedString>,
                checked: bool,
            ) -> Stateful<Div> {
                crate::button::root(id, label)
                    .role(Role::$role)
                    .aria_toggled(checked.into())
            }
            pub fn $part() -> Div {
                div()
            }
        }
    };
}
checkable!(switch, Switch, thumb);
checkable!(checkbox, CheckBox, indicator);
checkable!(toggle, Button, indicator);

macro_rules! group {
    ($module:ident, $role:ident) => {
        pub mod $module {
            use super::*;
            surface!(root, $role);
        }
    };
}
group!(checkbox_group, Group);
group!(toggle_group, Group);
group!(toolbar, Toolbar);
group!(form, Form);
pub mod fieldset {
    use super::*;
    surface!(root, Group);
    surface!(legend, Label);
}
group!(separator, Splitter);

pub mod accordion {
    use super::*;
    surface!(root, Group);
    surface!(item, Group);
    surface!(header, Heading);
    pub use crate::collapsible::{panel, trigger};
}

pub mod tabs {
    use super::*;
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
}

macro_rules! popup {
    ($module:ident, $role:ident) => {
        pub mod $module {
            use super::*;
            pub use crate::collapsible::{root, trigger};
            surface!(popup, $role);
            pub use crate::button::root as close;
            pub fn backdrop() -> Div {
                div()
            }
        }
    };
}
popup!(dialog, Dialog);
popup!(alert_dialog, AlertDialog);
popup!(drawer, Dialog);
popup!(popover, Group);
popup!(preview_card, Group);

pub mod tooltip {
    use super::*;
    surface!(popup, Tooltip);
    pub use crate::button::root as trigger;
}
pub mod menu {
    use super::*;
    pub use crate::collapsible::{root, trigger};
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
}
pub mod card {
    use super::*;
    surface!(root, Group);
    surface!(title, Heading);
    pub fn content() -> Div {
        div()
    }
}

pub mod context_menu {
    pub use super::menu::{
        group, group_label, item, popup, root, separator, submenu_root, submenu_trigger,
    };
    use super::*;
    /// Wrap any target; the listener receives the pointer position in window coordinates.
    pub fn trigger(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        open: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Stateful<Div> {
        div().id(id).aria_label(label).on_mouse_down(
            MouseButton::Right,
            move |event, window, cx| {
                cx.stop_propagation();
                open(event, window, cx);
            },
        )
    }
    /// Mount a styled popup above its target and fit it inside the window.
    pub fn positioner(position: Point<Pixels>, popup: impl IntoElement) -> impl IntoElement {
        deferred(anchored().position(position).snap_to_window().child(popup))
    }
}
pub mod menubar {
    use super::*;
    surface!(root, MenuBar);
    pub use super::menu::{item, popup, trigger};
}
pub mod navigation_menu {
    use super::*;
    surface!(root, Navigation);
    action!(link, Link);
}
pub mod select {
    use super::*;
    pub use crate::collapsible::{root, trigger};
    pub fn value() -> Div {
        div()
    }
    pub fn popup() -> Div {
        div()
    }
    pub fn item_text() -> Div {
        div()
    }
    surface!(list, ListBox);
    pub fn item(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        selected: bool,
    ) -> Stateful<Div> {
        crate::button::root(id, label)
            .role(Role::ListBoxOption)
            .aria_selected(selected)
    }
}
pub mod input {
    use super::*;
    /// Decorate the application's editor. The editor retains its input handler and IME.
    pub fn control(editor: Stateful<Div>, label: impl Into<SharedString>) -> Stateful<Div> {
        editor.role(Role::TextInput).aria_label(label)
    }
}
pub mod field {
    use super::*;
    surface!(root, Group);
    surface!(label, Label);
    surface!(description, Label);
    surface!(error, Alert);
    pub use super::input::control;
}
pub mod combobox {
    pub use super::select::{item, list, root, trigger};
    use super::*;
    pub fn input(
        editor: Stateful<Div>,
        label: impl Into<SharedString>,
        open: bool,
    ) -> Stateful<Div> {
        editor
            .role(Role::EditableComboBox)
            .aria_label(label)
            .aria_expanded(open)
    }
}
pub mod autocomplete {
    pub use super::combobox::{input, item, list, root};
}
pub mod otp_field {
    use super::*;
    surface!(root, Group);
    pub use super::input::control as input;
    pub fn slot() -> Div {
        div()
    }
}
macro_rules! range {
    ($module:ident, $role:ident) => {
        pub mod $module {
            use super::*;
            pub fn root(
                id: impl Into<ElementId>,
                label: impl Into<SharedString>,
                range: &crate::state::Range,
            ) -> Stateful<Div> {
                div()
                    .id(id)
                    .role(Role::$role)
                    .aria_label(label)
                    .aria_min_numeric_value(range.min())
                    .aria_max_numeric_value(range.max())
                    .aria_numeric_value(range.value())
            }
        }
    };
}
range!(meter, Meter);
range!(progress, ProgressIndicator);
pub mod slider {
    use super::*;
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
}
pub mod number_field {
    use super::*;
    surface!(root, Group);
    pub use crate::button::root as increment;
    pub use crate::button::root as decrement;
    pub fn input(
        editor: Stateful<Div>,
        label: impl Into<SharedString>,
        range: &crate::state::Range,
    ) -> Stateful<Div> {
        editor
            .role(Role::SpinButton)
            .aria_label(label)
            .aria_min_numeric_value(range.min())
            .aria_max_numeric_value(range.max())
            .aria_numeric_value(range.value())
    }
}
pub mod scroll_area {
    use super::*;
    pub fn root() -> Div {
        div().min_h_0().min_w_0()
    }
    pub fn viewport(id: impl Into<ElementId>) -> Stateful<Div> {
        div().id(id).overflow_scroll()
    }
}
pub mod avatar {
    use super::*;
    surface!(root, Image);
    pub fn fallback() -> Div {
        div()
    }
}
pub mod toast {
    use super::*;
    surface!(viewport, Region);
    surface!(root, Status);
    pub use crate::button::root as close;
}
