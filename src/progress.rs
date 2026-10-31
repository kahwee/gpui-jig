//! Progress component parts.
//!
//! Expose a bounded value with a human-readable count.
//!
//! ```rust
//! use gpui::{prelude::*, *};
//! use gpui_jig::progress;
//! let range = gpui_jig::state::Range::new(0., 8., 3.).unwrap();
//! let element = progress::root("upload", "Upload progress", &range).child("3 of 8 files");
//! ```
use gpui::{prelude::*, *};
range!(ProgressIndicator);
