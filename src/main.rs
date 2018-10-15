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
        self.curses.retain(|_, val| val > &mut 0);
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
        let text = commit.message_raw().unwrap().to_lowercase().to_string();
        let author = commit.author().name().unwrap().to_string();
        let index = authors
            .iter()
            .position(|i| i.name == author)
            .expect("Could not find author");
        let mut author = authors.get_mut(index).unwrap();
        for word in text.split_whitespace() {
            if naughty_word(word, &curses) {
                author.update_occurrence(word);
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
}
