// Shared constructors for semantic GPUI elements.
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
    ($role:ident, $part:ident) => {
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
    };
}

macro_rules! group {
    ($role:ident) => {
        surface!(root, $role);
    };
}

macro_rules! popup {
    ($role:ident) => {
        pub use crate::collapsible::{root, trigger};
        surface!(popup, $role);
        pub use crate::button::root as close;
        pub fn backdrop() -> Div {
            div()
        }
    };
}

macro_rules! range {
    ($role:ident) => {
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
    };
}
