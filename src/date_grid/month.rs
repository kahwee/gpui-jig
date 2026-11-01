//! Date arithmetic shared by grid rendering, keyboard input and month controls.
use chrono::{Datelike, Days, Months};
pub use chrono::{NaiveDate, Weekday};

/// A month with a configurable first weekday. Blank cells belong to no date.
pub struct Month {
    first: NaiveDate,
    offset: u32,
}
fn column(day: Weekday, start: Weekday) -> u32 {
    (day.num_days_from_monday() + 7 - start.num_days_from_monday()) % 7
}
impl Month {
    pub fn new(date: NaiveDate, start: Weekday) -> Self {
        let first = date.with_day(1).unwrap();
        Self {
            first,
            offset: column(first.weekday(), start),
        }
    }
    /// Only the weeks needed for this month, each containing exactly seven slots.
    pub fn weeks(&self) -> impl Iterator<Item = [Option<NaiveDate>; 7]> + '_ {
        (0..6)
            .map(|week| {
                std::array::from_fn(|col| {
                    let day = (week * 7 + col as u32 + 1).checked_sub(self.offset)?;
                    self.first.with_day(day)
                })
            })
            .take_while(|week| week.iter().any(Option::is_some))
    }
}
/// Follow calendar navigation keys. Page moves clamp to the destination month.
pub fn navigate(date: NaiveDate, start: Weekday, key: &str) -> Option<NaiveDate> {
    let col = column(date.weekday(), start) as u64;
    match key {
        "left" => date.checked_sub_days(Days::new(1)),
        "right" => date.checked_add_days(Days::new(1)),
        "up" => date.checked_sub_days(Days::new(7)),
        "down" => date.checked_add_days(Days::new(7)),
        "home" => date.checked_sub_days(Days::new(col)),
        "end" => date.checked_add_days(Days::new(6 - col)),
        "pageup" => date.checked_sub_months(Months::new(1)),
        "pagedown" => date.checked_add_months(Months::new(1)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_date_appears_once_in_the_correct_weekday_column() {
        for year in [1900, 2000, 2024, 2026] {
            for month in 1..=12 {
                for start in [Weekday::Mon, Weekday::Sun, Weekday::Thu] {
                    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
                    let mut expected = first;
                    for week in Month::new(first, start).weeks() {
                        for (col, date) in week.into_iter().enumerate() {
                            if let Some(date) = date {
                                assert_eq!(date, expected);
                                assert_eq!(col as u32, column(date.weekday(), start));
                                expected = expected.succ_opt().unwrap();
                            }
                        }
                    }
                    assert_eq!(expected.day(), 1);
                    assert_ne!(expected.month(), month);
                }
            }
        }
    }
    #[test]
    fn keyboard_crosses_boundaries_and_clamps_months() {
        let d = |m, day| NaiveDate::from_ymd_opt(2024, m, day).unwrap();
        for (from, key, to) in [
            (d(1, 31), "pagedown", d(2, 29)),
            (d(3, 1), "left", d(2, 29)),
            (d(3, 1), "home", d(2, 26)),
            (d(3, 1), "end", d(3, 3)),
        ] {
            assert_eq!(navigate(from, Weekday::Mon, key), Some(to));
        }
        assert_eq!(navigate(NaiveDate::MAX, Weekday::Sun, "right"), None);
        assert_eq!(navigate(NaiveDate::MIN, Weekday::Sun, "up"), None);
    }
}
