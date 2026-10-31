//! Card component parts.
//!
//! Group a title with persistent content. Apply spacing and borders to the root.
//!
//! ```rust
//! use gpui::{prelude::*, *};
//! use gpui_jig::card;
//!
//! let element = card::root("project", "Field notes")
//!     .child(card::title("title", "Field notes").child("Field notes"))
//!     .child(card::content().child("Last edited today"));
//! ```
use gpui::{prelude::*, *};
surface!(root, Group);
surface!(title, Heading);
pub fn content() -> Div {
    div()
}
