//! Checkbox group component parts.
use gpui::{prelude::*, *};
group!(Group);

/// Compute the state of a parent checkbox from the selectable children.
/// An empty group is unchecked. Exclude disabled children if they cannot
/// participate in the parent's select-all action.
///
/// ```
/// use gpui::Toggled;
/// use gpui_jig::checkbox_group::aggregate;
/// assert_eq!(aggregate([true, false]), Toggled::Mixed);
/// assert_eq!(aggregate([true, true]), Toggled::True);
/// assert_eq!(aggregate([false, false]), Toggled::False);
/// assert_eq!(aggregate([]), Toggled::False);
/// ```
pub fn aggregate(values: impl IntoIterator<Item = bool>) -> Toggled {
    let mut values = values.into_iter();
    let Some(first) = values.next() else {
        return Toggled::False;
    };
    if values.any(|value| value != first) {
        Toggled::Mixed
    } else {
        first.into()
    }
}
