#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![forbid(unsafe_code)]
extern crate git2;
#[macro_use]
extern crate structopt;

use git2::{Commit, Repository};
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use std::time::Instant;
use structopt::clap::AppSettings;
use structopt::StructOpt;

static CURSES: &str = include_str!("words.txt");

#[derive(StructOpt, Debug)]
#[structopt(
    name = "git anger-management",
    about = "Ever wondered how angry your commits are? Look no further...",
    raw(global_settings = "&[AppSettings::ColoredHelp]")
)]
struct Cli {
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
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

    fn author(&mut self, author_name: &str) -> &mut Author {
        if !self.authors.contains_key(author_name) {
            self.authors
                .entry(author_name.into())
                .or_insert_with(|| Author::new(author_name));
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

    fn update_occurrence(&mut self, curse: &str) {
        self.curses
            .get_mut(curse)
            .map(|c| *c += 1)
            .unwrap_or_else(|| {
                self.curses.insert(curse.to_owned(), 1);
            })
    }

    fn is_naughty(&self) -> bool {
        !self.curses.is_empty()
    }
}

fn main() -> Result<(), Box<Error>> {
    let start = Instant::now();
    let curses: HashSet<&str> = CURSES.lines().collect();
    let opt = Cli::from_args();
    let path = match opt.directory {
        Some(directory) => directory,
        None => env::current_dir()?,
    };
    let verbose = opt.verbose;

    let repo = Repository::open(&path)?;
    let commits = {
        let mut revwalk = repo.revwalk()?;
        let mut commits: Vec<Commit> = Vec::new();
        revwalk.push_head()?;
        for commit_id in revwalk {
            let commit = repo.find_commit(commit_id?)?;
            commits.push(commit);
        }
        commits
    };

    let mut repo = Repo::new(path.file_name().unwrap().to_str().unwrap());
    for commit in &commits {
        if let (Some(author_name), Some(commit_message)) = (
            commit.author().name(),
            commit.message().map(|w| w.to_lowercase()),
        ) {
            let mut curses_added = 0;
            {
                let author = repo.author(author_name);
                author.total_commits += 1;
                for word in split_into_clean_words(&commit_message) {
                    if naughty_word(word, &curses) {
                        author.total_curses += 1;
                        curses_added += 1;
                        author.update_occurrence(word);
                    }
                }
            }
            repo.total_commits += 1;
            repo.total_curses += curses_added;
        } else {
            eprintln!(
                "Skipping commit {:?} because either the commit author or message is missing",
                commit
            );
        }
    }

    let end = Instant::now();
    if verbose {
        println!(
            "Took {:?} to parse {}",
            end.duration_since(start),
            repo.name
        );
    }

    println!("{}", repo);
    for mut author in repo.authors.values() {
        if author.is_naughty() {
            println!("{}", author);
        }
    }

    Ok(())
}

fn split_into_clean_words(input: &str) -> impl Iterator<Item = &str> {
    input
        .split(|c: char| !char::is_alphabetic(c))
        .filter(|w| !w.is_empty())
}

fn naughty_word(word: &str, naughty_list: &HashSet<&str>) -> bool {
    naughty_list.contains(&word)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_naughty_words() {
        let curses: HashSet<&str> = CURSES.lines().collect();
        assert!(naughty_word("fuck", &curses));
        assert!(naughty_word("cyberfuckers", &curses));
        assert!(!naughty_word("pretty", &curses));
    }

    #[test]
    fn test_clean_word() {
        let words = split_into_clean_words("This! is a string: with, some. words in? it;");
        assert_eq!(
            vec!["This", "is", "a", "string", "with", "some", "words", "in", "it"],
            words.collect::<Vec<_>>()
        );
    }
}
