use crate::{
    author::Author,
    core::{naughty_word, split_into_clean_words},
};
use git2::{Commit, Repository};
use prettytable::{format, Cell, Row, Table};
use std::{collections::HashMap, env, error::Error, fmt, path::Path};

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
    pub fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        let repo = Repository::open(path)?;
        let commits = Repo::commits(&repo)?;

        let repo = match path.file_name() {
            Some(path) => path.to_str().unwrap().to_owned(),
            None => env::current_dir()?.to_str().unwrap().to_owned(),
        };

        let mut repo = Repo {
            name: repo.into(),
            total_commits: 0,
            total_curses: 0,
            curses: HashMap::new(),
            authors: HashMap::new(),
        };

        repo.build(commits);
        repo.count_curses();

        Ok(repo)
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

    /// Add curses that authors haven't muttered yet, for display purposes only
    pub fn add_missing(&mut self) {
        let curses = self.curses.clone();
        for (_, author) in self.authors.iter_mut() {
            author.add_missing(&curses);
        }
    }

    /// Count total naughty authors in repository.
    fn total_naughty_authors(&self) -> usize {
        self.authors.values().filter(|a| a.is_naughty()).count()
    }

    /// Build a table to display naughty authors and their words.
    fn build_table(&self) -> String {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        self.table_headers(&mut table);
        for author in self.authors.values() {
            if author.is_naughty() {
                Repo::add_author(&mut table, &author.curses, &author.name);
            }
        }
        if self.total_naughty_authors() > 1 {
            self.total_repo(&mut table);
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

    /// Add headers to a table
    fn table_headers(&self, table: &mut Table) {
        let curses = Repo::sort(&self.curses);
        let mut heading: Vec<_> = curses
            .clone()
            .into_iter()
            .map(|(name, _)| Cell::new(name))
            .collect();
        heading.insert(0, Cell::new("Author"));
        heading.push(Cell::new("Total"));
        table.set_titles(Row::new(heading));
    }

    /// Add an author to a table
    fn add_author(table: &mut Table, curses: &HashMap<String, usize>, author: &str) {
        let curses = Repo::sort(curses);
        let total: usize = curses.clone().into_iter().map(|(_, count)| count).sum();
        let mut curses: Vec<_> = curses
            .into_iter()
            .map(|(_, count)| Cell::new(&format!("{}", count)))
            .collect();
        curses.insert(0, Cell::new(author));
        curses.push(Cell::new(&format!("{}", total)));
        table.add_row(Row::new(curses));
    }

    /// Add the total amount of curses muttered in the repository
    fn total_repo(&self, table: &mut Table) {
        let curses = Repo::sort(&self.curses);
        let total: usize = curses.clone().into_iter().map(|(_, count)| count).sum();
        let mut curses: Vec<_> = curses
            .into_iter()
            .map(|(_, count)| Cell::new(&format!("{}", count)))
            .collect();
        curses.insert(0, Cell::new("Total"));
        curses.push(Cell::new(&format!("{}", total)));
        table.add_row(Row::new(curses));
    }

    /// Build a list of commits by walking the history of a repository.
    pub fn commits(repo: &Repository) -> Result<Vec<Commit>, Box<dyn Error>> {
        let mut revwalk = repo.revwalk()?;
        let mut commits: Vec<Commit> = Vec::new();
        revwalk.push_head()?;
        for commit_id in revwalk {
            let commit = repo.find_commit(commit_id?)?;
            commits.push(commit);
        }

        Ok(commits)
    }

    /// Documentation dammit!
    pub fn build(&mut self, commits: Vec<Commit>) {
        for commit in &commits {
            if let (Some(author_name), Some(commit_message)) = (
                commit.author().name(),
                commit.message().map(|w| w.to_lowercase()),
            ) {
                let mut curses_added = 0;
                {
                    let author = self.author(author_name);
                    author.total_commits += 1;
                    for word in split_into_clean_words(&commit_message) {
                        if naughty_word(word) {
                            author.total_curses += 1;
                            curses_added += 1;
                            author.update_occurrence(word);
                        }
                    }
                }
                self.total_commits += 1;
                self.total_curses += curses_added;
            } else {
                eprintln!(
                    "Skipping commit {:?} because either the commit author or message is missing",
                    commit
                );
            }
        }
    }
}
