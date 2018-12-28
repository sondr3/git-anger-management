use crate::curse::Curse;
use hashbrown::HashMap;
use std::fmt;

/// An author of a git commit.
#[derive(Debug)]
pub struct Author {
    /// Name of the author.
    pub name: String,
    /// Total count of commits by author.
    pub total_commits: usize,
    /// Total count of curses used by author.
    pub total_curses: usize,
    /// HashMap of all the curses the author used.
    pub curses: HashMap<String, Curse>,
}

impl fmt::Display for Author {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: ({}/{}) naughty commits/commits\n{}",
            self.name,
            self.total_curses,
            self.total_commits,
            Curse::table(&self.curses)
        )
    }
}

impl Author {
    /// Initialize a new author from a name.
    pub fn new(name: impl Into<String>) -> Self {
        Author {
            name: name.into(),
            curses: HashMap::new(),
            total_commits: 0,
            total_curses: 0,
        }
    }

    /// Update a previously used curse or add a new one.
    pub fn update_occurrence(&mut self, curse: &str) {
        self.curses
            .get_mut(curse)
            .map(|c| c.count += 1)
            .unwrap_or_else(|| {
                self.curses.insert(curse.into(), Curse::new(curse, 1));
            })
    }

    /// `git-anger-management` knows if you've been naughty or not
    pub fn is_naughty(&self) -> bool {
        !self.curses.is_empty()
    }
}
