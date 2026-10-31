//! Avatar component parts.
//!
//! Render initials while the application loads a profile image.
//!
//! ```rust
//! use gpui::{prelude::*, *};
//! use gpui_jig::avatar;
//!
//! let element = avatar::root("owner", "Project owner").child(avatar::fallback().child("JD"));
//! ```
use gpui::{prelude::*, *};
surface!(root, Image);
pub fn fallback() -> Div {
    div()
}
