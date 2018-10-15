#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![forbid(unsafe_code)]
extern crate git2;
#[macro_use]
extern crate structopt;

use git2::{Commit, Repository};
use std::cmp;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
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

#[derive(Debug, Eq)]
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
            "{}: ({}/{}) commits/naughty commits\n{:#?}",
            self.name, self.total_curses, self.total_commits, self.curses
        )
    }
}

impl cmp::PartialEq for Author {
    fn eq(&self, other: &Author) -> bool {
        self.name == other.name
    }
}

impl Author {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        let curses: Vec<&str> = CURSES.lines().collect();
        let mut map: HashMap<String, usize> = HashMap::new();
        for curse in curses {
            map.insert(curse.to_string(), 0);
        }
        Author {
            name,
            curses: map,
            total_commits: 0,
            total_curses: 0,
        }
    }

    fn did_a_naughty(&mut self) {
        self.total_curses += 1;
    }

    fn did_a_commit(&mut self) {
        self.total_commits += 1;
    }

    fn update_occurrence(&mut self, curse: &str) {
        self.curses.entry(curse.to_string()).and_modify(|i| *i += 1);
    }

    fn filter_occurrences(&mut self) {
        self.curses.retain(|_, val| *val > 0);
    }

    fn is_not_naughty(&self) -> bool {
        self.curses.is_empty()
    }
}

fn main() -> Result<(), Box<Error>> {
    let opt = Cli::from_args();
    let curses: Vec<&str> = CURSES.lines().collect();
    let path = if opt.directory.is_none() {
        env::current_dir()?
    } else {
        opt.directory.unwrap()
    };
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    let mut commits: Vec<Commit> = Vec::new();
    revwalk.push_head()?;
    for commit in revwalk {
        let commit = repo.find_commit(commit?)?;
        commits.push(commit);
    }
    let mut authors: Vec<Author> = find_authors(&commits);
    for commit in &commits {
        let text = commit.message().unwrap().to_lowercase().to_string();
        let author = commit.author().name().unwrap().to_string();
        let index = authors
            .iter()
            .position(|i| i.name == author)
            .expect("Could not find author");
        let mut author = &mut authors[index];
        author.did_a_commit();
        for word in text.split_whitespace() {
            let word = clean_word(word);
            if naughty_word(word.as_str(), &curses) {
                author.did_a_naughty();
                author.update_occurrence(word.as_str());
            }
        }
    }
    for mut author in authors {
        author.filter_occurrences();
        if !author.is_not_naughty() {
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

fn naughty_word(word: &str, naughty_list: &[&str]) -> bool {
    naughty_list.contains(&word)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_naughty_words() {
        let curses: Vec<&str> = CURSES.lines().collect();
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
