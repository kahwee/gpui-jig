//! Input component parts.
use gpui::{prelude::*, *};
/// Decorate the application's editor. The editor retains its input handler and IME.
pub fn control(editor: Stateful<Div>, label: impl Into<SharedString>) -> Stateful<Div> {
    editor.role(Role::TextInput).aria_label(label)
}
