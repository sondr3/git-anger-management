use author::Author;
use hashbrown::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Repo {
    pub name: String,
    pub total_commits: usize,
    pub total_curses: usize,
    pub curses: HashMap<String, usize>,
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
    pub fn new(name: impl Into<String>) -> Self {
        Repo {
            name: name.into(),
            total_commits: 0,
            total_curses: 0,
            curses: HashMap::new(),
            authors: HashMap::new(),
        }
    }

    pub fn author(&mut self, author_name: &str) -> &mut Author {
        if !self.authors.contains_key(author_name) {
            self.authors
                .entry(author_name.into())
                .or_insert_with(|| Author::new(author_name));
        }

        self.authors.get_mut(author_name).expect("exists")
    }

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
