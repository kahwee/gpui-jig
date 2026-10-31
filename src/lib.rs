#![doc = include_str!("../README.md")]
//! Unstyled, composable GPUI components for Rust.
pub mod button;
pub mod collapsible;
pub mod radio_group;

mod parts;
pub use parts::*;
pub mod state;
