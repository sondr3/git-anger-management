#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
extern crate git2;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate lazy_static;

use git2::Repository;
use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use structopt::clap::AppSettings;
use structopt::StructOpt;

static CURSES: &str = include_str!("words.txt");
lazy_static! {
    static ref CURSES_SET: HashSet<&'static str> = { CURSES.lines().collect() };
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "git anger-management",
    about = "Ever wondered how angry your commits are? Look no further...",
    raw(global_settings = "&[AppSettings::ColoredHelp]")
)]
struct Cli {
    #[structopt(
        name = "directory",
        help = "Directory to parse commits",
        parse(from_os_str)
    )]
    directory: Option<PathBuf>,
}

#[derive(Debug)]
struct Repo {
    name: String,
    total_commits: usize,
    total_curses: usize,
    authors: HashMap<String, Author>,
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: ({}/{}) naughty commits/commits",
            self.name, self.total_curses, self.total_commits
        )
    }
}

impl Repo {
    fn new(name: impl Into<String>) -> Self {
        Repo {
            name: name.into(),
            total_commits: 0,
            total_curses: 0,
            authors: HashMap::new(),
        }
    }

    fn author_for(&mut self, author_name: &str) -> &mut Author {
        if !self.authors.contains_key(author_name) {
            self.authors
                .insert(author_name.to_owned(), Author::new(author_name));
        }

        self.authors.get_mut(author_name).expect("exists")
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Author {
    name: String,
    total_commits: usize,
    total_curses: usize,
    curses: HashMap<String, usize>,
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
    fn new(name: impl Into<String>) -> Self {
        Author {
            name: name.into(),
            curses: HashMap::new(),
            total_commits: 0,
            total_curses: 0,
        }
    }

    fn update_occurrence(&mut self, curse: String) {
        *self.curses.entry(curse).or_insert(0) += 1;
    }

    fn is_naughty(&self) -> bool {
        !self.curses.is_empty()
    }
}

fn main() -> Result<(), Box<Error>> {
    let repo_path = {
        let opt = Cli::from_args();
        if let Some(directory) = opt.directory {
            directory
        } else {
            env::current_dir()?
        }
    };

    let repo = Repository::open(&repo_path)?;
    let commits = {
        let mut revwalk = repo.revwalk()?;
        let mut commits = Vec::new();
        revwalk.push_head()?;
        for commit_id in revwalk {
            let commit = repo.find_commit(commit_id?)?;
            commits.push(commit);
        }
        commits
    };

    let mut repo = Repo::new(repo_path.file_name().unwrap().to_str().unwrap());
    for commit in &commits {
        if let (Some(author_name), Some(commit_message)) = (
            commit.author().name(),
            commit.message().map(|msg| msg.to_lowercase()),
        ) {
            let mut total_curses_added = 0;

            {
                let author = repo.author_for(author_name);
                author.total_commits += 1;
                for word in commit_message.split_whitespace() {
                    let word = clean_word(word);
                    if naughty_word(&word) {
                        author.total_curses += 1;
                        total_curses_added += 1;
                        author.update_occurrence(word);
                    }
                }
            }

            repo.total_commits += 1;
            repo.total_curses += total_curses_added;
        } else {
            println!(
                "Warning: skipping commit {:?} because author name OR commit message missing",
                commit
            );
        }
    }

    println!("{}", repo);
    for mut author in repo.authors.values() {
        if author.is_naughty() {
            println!("{}", author);
        }
    }
    Ok(())
}

fn clean_word(word: &str) -> String {
    let mut res = String::with_capacity(word.len());
    for b in word.chars() {
        match b {
            '!' => {}
            '?' => {}
            ':' => {}
            ';' => {}
            '.' => {}
            ',' => {}
            _ => res.push(b),
        }
    }
    res
}

fn naughty_word(word: &str) -> bool {
    CURSES_SET.contains(&word)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_naughty_words() {
        assert!(naughty_word("fuck"));
        assert!(naughty_word("cyberfuckers"));
        assert!(!naughty_word("pretty"));
    }

    #[test]
    fn test_clean_word() {
        let w1 = "This! is a string: with, some. words in? it;".to_string();
        let w1 = clean_word(w1.as_str());
        assert_eq!(
            "This is a string with some words in it",
            w1.trim().to_string()
        );
    }
}
