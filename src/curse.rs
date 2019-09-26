use prettytable::{format, Cell, Row, Table};
use std::collections::HashMap;
use std::fmt;

/// A single curse with a count of how many times it occurs
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct Curse {
    /// How many times have this curse been invoked?
    pub count: usize,
    /// The curses name
    pub curse: String,
}

impl fmt::Display for Curse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.curse, self.count)
    }
}

impl Curse {
    /// Creates a new curse with a name and a count
    pub fn new(curse: impl Into<String>, count: usize) -> Self {
        Curse {
            curse: curse.into(),
            count,
        }
    }

    /// Create a sorted `Vec` from a HashMap of curses, sorted by counts
    fn sort(curses: &HashMap<String, Curse>) -> Vec<&Curse> {
        let mut curses: Vec<_> = curses.iter().map(|c| c.1).collect();
        curses.sort_unstable();
        curses.reverse();
        curses
    }

    /// Create a pretty table that prints curses and their counts
    pub fn table(curses: &HashMap<String, Curse>) -> Table {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        let curses = Curse::sort(curses);

        // jesus this is so ugly
        let mut heading: Vec<_> = curses
            .clone()
            .into_iter()
            .map(|c| Cell::new(c.curse.as_str()))
            .collect();
        heading.push(Cell::new("Total"));
        heading.insert(0, Cell::new(""));
        table.set_titles(Row::new(heading));

        let total: usize = curses.clone().into_iter().map(|c| c.count).sum();
        let mut curses: Vec<_> = curses
            .into_iter()
            .map(|c| Cell::new(&format!("{}", c.count)))
            .collect();
        curses.push(Cell::new(&format!("{}", total)));
        curses.insert(0, Cell::new("Overall"));
        table.add_row(Row::new(curses));

        table
    }
}
