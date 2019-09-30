use crate::author::Author;
use prettytable::{format, Cell, Row, Table};
use std::collections::HashMap;
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
        write!(f, "{}", self.build_table())
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
            for (name, curse) in &author.curses {
                self.curses
                    .entry(name.to_string())
                    .and_modify(|c| *c += *curse)
                    .or_insert_with(|| *curse);
            }
        }
    }

    /// wat
    pub fn add_missing(&mut self) {
        let curses = self.curses.clone();
        for (_, author) in self.authors.iter_mut() {
            author.add_missing(&curses);
        }
    }

    fn build_table(&self) -> String {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_CLEAN);
        Repo::table_headers(&mut table, &self.curses);
        for author in self.authors.values() {
            if author.is_naughty() {
                Repo::add_author(&mut table, &author.curses, &author.name);
            }
        }

        table.to_string()
    }

    /// Create a sorted `Vec` from a HashMap of curses, sorted by counts
    fn sort(curses: &HashMap<String, usize>) -> Vec<(&String, &usize)> {
        let mut curses: Vec<(&String, &usize)> = curses.iter().collect();
        curses.sort_by(|a, b| a.1.cmp(b.1));
        curses.reverse();
        curses
    }

    fn table_headers(table: &mut Table, curses: &HashMap<String, usize>) {
        let curses = Repo::sort(curses);
        let mut heading: Vec<_> = curses
            .clone()
            .into_iter()
            .map(|(name, _)| Cell::new(name))
            .collect();
        heading.push(Cell::new("Total"));
        heading.insert(0, Cell::new(""));
        table.set_titles(Row::new(heading));
    }

    fn add_author(table: &mut Table, curses: &HashMap<String, usize>, author: &str) {
        let curses = Repo::sort(curses);
        let total: usize = curses.clone().into_iter().map(|(_, count)| count).sum();
        let mut curses: Vec<_> = curses
            .into_iter()
            .map(|(_, count)| Cell::new(&format!("{}", count)))
            .collect();
        curses.push(Cell::new(&format!("{}", total)));
        curses.insert(0, Cell::new(author));
        table.add_row(Row::new(curses));
    }
}
