//! Switch component parts.
//!
//! Compose [`root`] and [`thumb`] and style each with GPUI modifiers. Keep
//! the checked value in your view and use it for both semantics and styling.
//! Attach a listener to update the value and notify the view.
//!
//! ```rust
//! use gpui::{prelude::*, *};
//! use gpui_jig::switch;
//!
//! let checked = true;
//! let element = switch::root("notifications", "Notifications", checked)
//!     .w(px(36.)).h(px(20.)).p(px(2.))
//!     .flex().items_center().rounded_full()
//!     .when(checked, |s| s.justify_end())
//!     .bg(rgb(if checked { 0x20704a } else { 0x707070 }))
//!     .child(switch::thumb().size(px(16.)).rounded_full().bg(rgb(0xffffff)));
//! ```
//!
//! The root supplies a role, accessible name and checked state. The thumb is
//! an unstyled `Div`. The application supplies state changes and focus styling.
use gpui::{prelude::*, *};
checkable!(Switch, thumb);
