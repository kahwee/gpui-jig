//! Toggle group component parts.
use gpui::{prelude::*, *};
group!(Group);

/// A focusable group item. The owner updates its pressed state on activation
/// and decides whether one or multiple items may be pressed.
pub use crate::toggle::root as item;
