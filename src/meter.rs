//! Meter component parts.
use gpui::{prelude::*, *};
range!(Meter);

/// Container for the full extent of a measurement.
pub fn track() -> Div {
    div()
}
/// Fill styled by the caller to represent the measured fraction.
pub fn indicator() -> Div {
    div()
}
/// Human-readable measurement, such as "6 GB of 10 GB".
pub fn value() -> Div {
    div()
}
