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
    fn new(name: &str) -> Self {
        let name = name.to_string();
        Repo {
            name,
            total_commits: 0,
            total_curses: 0,
        }
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
    fn new(name: &str) -> Self {
        let name = name.to_string();
        let curses: HashMap<String, usize> = HashMap::new();
        Author {
            name,
            curses,
            total_commits: 0,
            total_curses: 0,
        }
    }

    fn update_occurrence(&mut self, curse: &str) {
        if !self.curses.contains_key(curse) {
            self.curses.insert(curse.to_string(), 1);
        } else {
            self.curses.entry(curse.to_string()).and_modify(|i| *i += 1);
        }
    }

    fn is_naughty(&self) -> bool {
        !self.curses.is_empty()
    }
}

fn main() -> Result<(), Box<Error>> {
    let start = Instant::now();
    let opt = Cli::from_args();
    let curses: HashSet<&str> = CURSES.lines().collect();
    let path = if opt.directory.is_none() {
        env::current_dir()?
    } else {
        opt.directory.unwrap()
    };
    let repo = Repository::open(&path)?;
    let mut revwalk = repo.revwalk()?;
    let mut commits: Vec<Commit> = Vec::new();
    revwalk.push_head()?;
    for commit in revwalk {
        let commit = repo.find_commit(commit?)?;
        commits.push(commit);
    }
    let mut repo = Repo::new(path.file_name().unwrap().to_str().unwrap());
    let mut authors: Vec<Author> = find_authors(&commits);
    for commit in &commits {
        let author_name = commit
            .author()
            .name()
            .map(|n| n.to_owned())
            .expect("No author found");
        let commit_message = commit
            .message()
            .map(|msg| msg.to_lowercase())
            .expect("No commit message found");
        let index = authors
            .iter()
            .position(|i| i.name == author_name)
            .expect("Could not find author");
        let mut author = &mut authors[index];
        author.total_commits += 1;
        repo.total_commits += 1;
        for word in commit_message.split_whitespace() {
            let word = clean_word(word);
            if naughty_word(word.as_str(), &curses) {
                author.total_curses += 1;
                repo.total_curses += 1;
                author.update_occurrence(word.as_str());
            }
        }
    }
    let end = Instant::now();
    println!("{:?}", end.duration_since(start));
    println!("{}", repo);
    for mut author in authors {
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

fn find_authors(commits: &[Commit]) -> Vec<Author> {
    let mut names: Vec<String> = Vec::new();
    let mut res: Vec<Author> = Vec::new();
    for commit in commits {
        let name = commit.author().name().unwrap().to_string();
        if !names.contains(&name) {
            res.push(Author::new(name.as_str()));
        }
        names.push(name);
    }
    res
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
        let w1 = "This! is a string: with, some. words in? it;".to_string();
        let w1 = clean_word(w1.as_str());
        assert_eq!(
            "This is a string with some words in it",
            w1.trim().to_string()
        );
    }
}
