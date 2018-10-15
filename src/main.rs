#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![forbid(unsafe_code)]
extern crate git2;

use git2::{Commit, Repository};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::env;
use std::error::Error;

static CURSES: &str = include_str!("words.txt");

#[derive(Debug, Eq)]
struct Author {
    name: String,
    curses: HashMap<String, usize>,
}

impl PartialEq for Author {
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
        Author { name, curses: map }
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
    let curses: Vec<&str> = CURSES.lines().collect();
    let path = env::current_dir()?;
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
        for word in text.split_whitespace() {
            let word = clean_word(word);
            if naughty_word(word.as_str(), &curses) {
                author.update_occurrence(word.as_str());
            }
        }
    }
    for mut author in authors {
        author.filter_occurrences();
        if !author.is_not_naughty() {
            println!("{}", author.name);
            println!("{:#?}", author.curses);
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
