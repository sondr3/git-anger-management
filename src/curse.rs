use hashbrown::HashMap;
use prettytable::{cell, format, row, Table};
use std::fmt;

/// A single curse with a count of how many times it occurs
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
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
        let mut curses: Vec<_> = curses.into_iter().map(|c| c.1).collect();
        curses.sort_unstable();
        curses.reverse();
        curses
    }

    /// Create a pretty table that prints curses and their counts
    pub fn table(curses: &HashMap<String, Curse>) -> Table {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        let curses = Curse::sort(curses);

        table.set_titles(row!["Curse", "Count"]);
        for curse in curses {
            table.add_row(row![curse.curse, curse.count]);
        }

        table
    }
}
