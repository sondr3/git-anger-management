use hashbrown::HashMap;
use prettytable::{cell, format, row, Table};
use std::fmt;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Curse {
    pub count: usize,
    pub curse: String,
}

impl fmt::Display for Curse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.curse, self.count)
    }
}

impl Curse {
    pub fn new(curse: impl Into<String>, count: usize) -> Self {
        Curse {
            curse: curse.into(),
            count,
        }
    }

    fn sort(curses: &HashMap<String, Curse>) -> Vec<&Curse> {
        let mut curses: Vec<_> = curses.into_iter().map(|c| c.1).collect();
        curses.sort_unstable();
        curses.reverse();
        curses
    }

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
