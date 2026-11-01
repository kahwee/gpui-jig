//! Unstyled table parts and optional sorting for borrowed rows.
//!
//! The view owns its data, layout, selection, and keyboard interaction. Define
//! columns once, use [`Sorting::toggle`] to cycle the order, and render the
//! returned row references with [`ColumnDef::cell`]. Equal keys retain their
//! input order. The constructors supply table roles without imposing styles.
//!
//! ```rust
//! use gpui_jig::table::{ColumnDef, Sorting};
//! let data = [12, 3, 8];
//! let columns = [ColumnDef::accessor("Builds", |n: &u32| n)];
//! let mut sorting = Sorting::default();
//! sorting.toggle(0);
//! assert_eq!(sorting.rows(&data, &columns), vec![&3, &8, &12]);
//! ```
use gpui::{Div, ElementId, Role, SharedString, Stateful, div, prelude::*};
use std::{cmp::Ordering, fmt::Display};

/// A named table container.
pub fn root(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    div().id(id).role(Role::Table).aria_label(label)
}

/// A table row. The caller supplies its contents and layout.
pub fn row(id: impl Into<ElementId>) -> Stateful<Div> {
    div().id(id).role(Role::Row)
}

/// A column heading; place a separate button inside it for sorting.
pub fn column_header(id: impl Into<ElementId>) -> Stateful<Div> {
    div().id(id).role(Role::ColumnHeader)
}

/// A named cell. Keep the label in sync with the visible cell value.
pub fn cell(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Stateful<Div> {
    div().id(id).role(Role::Cell).aria_label(label)
}

type Compare<Row> = dyn Fn(&Row, &Row) -> Ordering;

pub struct ColumnDef<Row> {
    pub header: &'static str,
    compare: Box<Compare<Row>>,
    format: Box<dyn Fn(&Row) -> String>,
}

impl<Row: 'static> ColumnDef<Row> {
    /// Sort by the field's native type and use its display representation for cells.
    pub fn accessor<Value: Ord + Display + ?Sized + 'static>(
        header: &'static str,
        get: fn(&Row) -> &Value,
    ) -> Self {
        Self {
            header,
            compare: Box::new(move |a, b| get(a).cmp(get(b))),
            format: Box::new(move |row| get(row).to_string()),
        }
    }

    /// Customize cell text without changing the column's ordering.
    pub fn format_cell(mut self, format: impl Fn(&Row) -> String + 'static) -> Self {
        self.format = Box::new(format);
        self
    }

    pub fn cell(&self, row: &Row) -> String {
        (self.format)(row)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Ascending,
    Descending,
}

#[derive(Default)]
pub struct Sorting(Option<(usize, Direction)>);

impl Sorting {
    pub fn direction(&self, column: usize) -> Option<Direction> {
        self.0
            .filter(|(index, _)| *index == column)
            .map(|(_, direction)| direction)
    }

    /// Cycle through ascending, descending and original input order.
    pub fn toggle(&mut self, column: usize) {
        self.0 = match self.direction(column) {
            None => Some((column, Direction::Ascending)),
            Some(Direction::Ascending) => Some((column, Direction::Descending)),
            Some(Direction::Descending) => None,
        };
    }

    pub fn rows<'a, Row>(&self, data: &'a [Row], columns: &[ColumnDef<Row>]) -> Vec<&'a Row> {
        let mut rows: Vec<_> = data.iter().collect();
        if let Some((index, direction)) = self.0
            && let Some(column) = columns.get(index)
        {
            rows.sort_by(|a, b| {
                let order = (column.compare)(a, b);
                match direction {
                    Direction::Ascending => order,
                    Direction::Descending => order.reverse(),
                }
            });
        }
        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_sort_preserves_ties_and_cycles_back_to_input_order() {
        let data = [(12, "a"), (2, "b"), (12, "c")];
        let columns = [ColumnDef::accessor("Count", |row: &(i32, &str)| &row.0)
            .format_cell(|row| format!("{} builds", row.0))];
        let mut sort = Sorting::default();
        for expected in [["b", "a", "c"], ["a", "c", "b"], ["a", "b", "c"]] {
            sort.toggle(0);
            let names: Vec<_> = sort.rows(&data, &columns).iter().map(|row| row.1).collect();
            assert_eq!(names, expected);
        }
        assert_eq!(columns[0].cell(&data[0]), "12 builds");
    }
}
