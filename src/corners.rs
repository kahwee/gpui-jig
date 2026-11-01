//! Optional corner tokens in logical pixels. Primitives do not apply them automatically.
//! Set individual values to override a preset, then use GPUI's `.rounded(px(...))`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Corners {
    pub control: u16,
    pub card: u16,
    pub dialog: u16,
    pub toast: u16,
}
impl Corners {
    pub const SQUARE: Self = Self {
        control: 0,
        card: 0,
        dialog: 0,
        toast: 0,
    };
    pub const STANDARD: Self = Self {
        control: 6,
        card: 10,
        dialog: 14,
        toast: 10,
    };
    pub const ROUND: Self = Self {
        control: 12,
        card: 18,
        dialog: 22,
        toast: 16,
    };
    /// Radius for an aligned child inset equally from both edges of its parent.
    pub const fn inset(outer: u16, inset: u16) -> u16 {
        outer.saturating_sub(inset)
    }
}
impl Default for Corners {
    fn default() -> Self {
        Self::STANDARD
    }
}
