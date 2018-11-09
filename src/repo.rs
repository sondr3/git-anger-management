use author::Author;
use hashbrown::HashMap;
use std::fmt;

/// A simple representation of a git repository.
#[derive(Debug)]
pub struct Repo {
    /// Name of the repository.
    pub name: String,
    /// Count of the total amount of commits in the repository.
    pub total_commits: usize,
    /// Count of the total amount of curses used in the commits.
    pub total_curses: usize,
    /// HashMap of all the naughty words used by the authors.
    pub curses: HashMap<String, usize>,
    /// HashMap of all the authors that have been committed.
    pub authors: HashMap<String, Author>,
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: ({}/{}) naughty commits/commits\n{:#?}",
            self.name, self.total_curses, self.total_commits, self.curses
        )
    }
}

impl Repo {
    /// Creates a new and empty repository.
    pub fn new(name: impl Into<String>) -> Self {
        Repo {
            name: name.into(),
            total_commits: 0,
            total_curses: 0,
            curses: HashMap::new(),
            authors: HashMap::new(),
        }
    }

    /// Checks if an author exists and creates a new author if she/he doesn't
    /// exist.
    pub fn author(&mut self, author_name: &str) -> &mut Author {
        if !self.authors.contains_key(author_name) {
            self.authors
                .entry(author_name.into())
                .or_insert_with(|| Author::new(author_name));
        }

        self.authors.get_mut(author_name).expect("exists")
    }

    /// Counts all the naughty words used by authors.
    pub fn count_curses(&mut self) {
        for author in self.authors.values() {
            for (curse, count) in &author.curses {
                self.curses
                    .entry(curse.to_string())
                    .and_modify(|c| *c += count)
                    .or_insert(*count);
            }
        }
    }
}
