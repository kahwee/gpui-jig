//! Optional state for controlled components. No colors, dimensions or typography.
use std::collections::BTreeSet;

/// Expanded item keys for an accordion.
pub struct Accordion<K> {
    multiple: bool,
    open: BTreeSet<K>,
}
impl<K: Ord> Accordion<K> {
    pub fn new(multiple: bool) -> Self {
        Self {
            multiple,
            open: BTreeSet::new(),
        }
    }
    pub fn is_open(&self, key: &K) -> bool {
        self.open.contains(key)
    }
    pub fn toggle(&mut self, key: K) {
        if self.open.remove(&key) {
            return;
        }
        if !self.multiple {
            self.open.clear();
        }
        self.open.insert(key);
    }
}

/// Bounded numeric value shared by meters, progress, sliders and number fields.
pub struct Range {
    min: f64,
    max: f64,
    value: f64,
}
impl Range {
    pub fn new(min: f64, max: f64, value: f64) -> Option<Self> {
        if !min.is_finite() || !max.is_finite() || !value.is_finite() || min > max {
            return None;
        }
        Some(Self {
            min,
            max,
            value: value.clamp(min, max),
        })
    }
    pub fn min(&self) -> f64 {
        self.min
    }
    pub fn max(&self) -> f64 {
        self.max
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn set(&mut self, value: f64) {
        if value.is_finite() {
            self.value = value.clamp(self.min, self.max);
        }
    }
    pub fn step(&mut self, delta: f64) {
        self.set(self.value + delta);
    }
}

/// Move the active item in a list, menu or tab strip. Disabled items are skipped.
/// The caller commits a selection separately (e.g. on Enter for a listbox).
pub fn navigate(key: &str, current: Option<usize>, enabled: &[bool]) -> Option<usize> {
    let n = enabled.len();
    let forward = match key {
        "home" => return enabled.iter().position(|&e| e),
        "end" => return enabled.iter().rposition(|&e| e),
        "down" | "right" => true,
        "up" | "left" => false,
        _ => return current.filter(|&i| enabled.get(i) == Some(&true)),
    };
    if n == 0 {
        return None;
    }
    let start = current
        .filter(|&i| i < n)
        .unwrap_or(if forward { n - 1 } else { 0 });
    (1..=n)
        .map(|d| {
            if forward {
                (start + d) % n
            } else {
                (start + n - d) % n
            }
        })
        .find(|&i| enabled[i])
}

/// Tracks the opener so a dialog, drawer or popup can restore keyboard focus.
#[derive(Default)]
pub struct PopupFocus {
    open: bool,
    opener: Option<gpui::FocusHandle>,
}
impl PopupFocus {
    pub fn is_open(&self) -> bool {
        self.open
    }
    pub fn open(
        &mut self,
        first: &gpui::FocusHandle,
        window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) {
        if self.open {
            return;
        }
        self.opener = window.focused(cx);
        self.open = true;
        window.focus(first, cx);
    }
    pub fn close(&mut self, window: &mut gpui::Window, cx: &mut gpui::App) {
        self.open = false;
        if let Some(handle) = self.opener.take() {
            window.focus(&handle, cx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn navigation_wraps_and_skips_disabled_items() {
        let items = [true, false, true];
        assert_eq!(navigate("down", Some(0), &items), Some(2));
        assert_eq!(navigate("right", Some(2), &items), Some(0));
        assert_eq!(navigate("up", None, &items), Some(2));
        assert_eq!(navigate("home", Some(2), &[false, false]), None);
        assert_eq!(navigate("left", None, &[]), None);
    }
    #[test]
    fn accordion_controls_single_and_multiple_panels() {
        for multiple in [false, true] {
            let mut s = Accordion::new(multiple);
            s.toggle("a");
            s.toggle("b");
            assert_eq!(s.is_open(&"a"), multiple);
            s.toggle("b");
            assert!(!s.is_open(&"b"));
        }
    }
    #[test]
    fn range_rejects_invalid_bounds_and_clamps_changes() {
        assert!(Range::new(2., 1., 0.).is_none());
        let mut r = Range::new(0., 10., 4.).unwrap();
        r.step(20.);
        r.set(f64::NAN);
        assert_eq!(r.value(), 10.);
    }
}
