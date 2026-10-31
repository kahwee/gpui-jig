//! Scroll area component parts.
//!
//! Constrain the viewport height to scroll the file list independently of the window.
//!
//! ```rust
//! use gpui::{prelude::*, *};
//! use gpui_jig::scroll_area;
//!
//! let element = scroll_area::root().child(scroll_area::viewport("files").h(px(160.)).children(
//!     (1..=30).map(|n| div().child(format!("Document {n}"))),
//! ));
//! ```
use gpui::{prelude::*, *};
pub fn root() -> Div {
    div().min_h_0().min_w_0()
}
pub fn viewport(id: impl Into<ElementId>) -> Stateful<Div> {
    div().id(id).overflow_scroll()
}
