use hashbrown::HashMap;
use std::fmt;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Curse {
    pub count: usize,
    pub curse: String,
}

impl fmt::Display for Curse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\t{}: {}", self.curse, self.count)
    }
}

impl Curse {
    pub fn new(curse: impl Into<String>, count: usize) -> Self {
        Curse {
            curse: curse.into(),
            count,
        }
    }

    pub fn sort(curses: &HashMap<String, Curse>) -> String {
        let mut curses: Vec<_> = curses.iter().map(|c| c.1).collect();
        curses.sort_unstable();
        curses.reverse();
        let mut result = String::new();
        for curse in &curses {
            result.push_str(&format!("{}\n", curse));
        }
        result
    }
}
