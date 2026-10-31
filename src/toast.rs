//! Toast component parts.
//!
//! Mount a notification when an operation finishes. The view controls the timer and close listener.
//!
//! ```rust
//! use gpui::{prelude::*, *};
//! use gpui_jig::toast;
//!
//! let element = toast::viewport("notifications", "Notifications").child(
//!     toast::root("saved", "Document saved").child("Document saved")
//!         .child(toast::close("dismiss", "Dismiss notification").child("Dismiss")),
//! );
//! ```
use gpui::{prelude::*, *};
surface!(viewport, Region);
surface!(root, Status);
pub use crate::button::root as close;

/// Short summary of the notification within its status region.
pub fn title() -> Div {
    div()
}
/// Supporting detail within the same notification.
pub fn description() -> Div {
    div()
}
