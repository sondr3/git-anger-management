use std::{collections::HashMap, env, error::Error, path::Path};

use git2::{Commit, Repository};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{
    author::Author,
    core::{naughty_word, split_into_clean_words},
};

/// A simple representation of a git repository.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
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
            name: repo,
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

    /// Count total naughty authors in repository.
    pub fn total_naughty_authors(&self) -> usize {
        self.authors.values().filter(|a| a.is_naughty()).count()
    }

    /// Build a list of commits by walking the history of a repository.
    pub fn commits(repo: &Repository) -> Result<Vec<Commit<'_>>, Box<dyn Error>> {
        let mut revwalk = repo.revwalk()?;
        let mut commits: Vec<Commit> = Vec::new();
        revwalk.push_head()?;
        for commit_id in revwalk {
            let commit = repo.find_commit(commit_id?)?;
            commits.push(commit);
        }

        Ok(commits)
    }

    /// Iterate over all commits, finding authors who have been naughty and
    /// keep track of them.
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
