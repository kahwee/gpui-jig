//! Accordion component parts.
//!
//! Use one expansion value for the trigger and the presence of its panel.
//!
//! ```rust
//! use gpui::{prelude::*, *};
//! use gpui_jig::accordion;
//! let open = true;
//! let element = accordion::item("install", "Installation")
//!     .child(accordion::header("heading", "Installation").child(
//!         accordion::trigger("toggle", "Installation", open).child("Installation"),
//!     ))
//!     .when(open, |s| s.child(accordion::panel().child("Add the library to Cargo.toml")));
//! ```
use gpui::{prelude::*, *};
surface!(root, Group);
surface!(item, Group);
surface!(header, Heading);
pub use crate::collapsible::{panel, trigger};
