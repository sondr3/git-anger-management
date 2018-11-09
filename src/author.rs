use hashbrown::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Author {
    pub name: String,
    pub total_commits: usize,
    pub total_curses: usize,
    pub curses: HashMap<String, usize>,
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: ({}/{}) naughty commits/commits\n{:#?}",
            self.name, self.total_curses, self.total_commits, self.curses
        )
    }
}

impl Author {
    pub fn new(name: impl Into<String>) -> Self {
        Author {
            name: name.into(),
            curses: HashMap::new(),
            total_commits: 0,
            total_curses: 0,
        }
    }

    pub fn update_occurrence(&mut self, curse: &str) {
        self.curses
            .get_mut(curse)
            .map(|c| *c += 1)
            .unwrap_or_else(|| {
                self.curses.insert(curse.to_owned(), 1);
            })
    }

    pub fn is_naughty(&self) -> bool {
        !self.curses.is_empty()
    }
}
