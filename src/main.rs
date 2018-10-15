#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![forbid(unsafe_code)]
extern crate git2;

use git2::{Commit, Repository};
use std::collections::HashMap;
use std::env;
use std::error::Error;

static CURSES: &str = include_str!("words.txt");

#[derive(Debug)]
struct Author {
    name: String,
    curses: HashMap<&'static str, usize>,
}

impl Author {
    fn new(name: &str, curses: HashMap<&'static str, usize>) -> Author {
        let name = name.to_string();
        Author {
            name, curses
        }
    }
}

fn main() -> Result<(), Box<Error>> {
    let curses: Vec<&str> = CURSES.lines().collect();
    let path = env::current_dir()?;
    let repo = Repository::open(path)?;
    let mut revwalk = repo.revwalk()?;
    let mut occurrences: HashMap<&str, usize> = HashMap::new();
    let mut commits: Vec<Commit> = Vec::new();
    for curse in &curses {
        occurrences.entry(curse).or_insert(0);
    }
    revwalk.push_head()?;
    for commit in revwalk {
        let commit = repo.find_commit(commit?)?;
        commits.push(commit);
    }
    let mut authors: Vec<Author> = find_authors(&commits);
    println!("{:?}", repo.workdir());
    filter_occurrences(&mut occurrences);
    println!("{:#?}", occurrences);
    println!("{:#?}", commits);
    print!("{:#?}", authors);
    Ok(())
}

fn find_authors(commits: &[Commit]) -> Vec<Author> {
    let mut names: Vec<String> = Vec::new();
    let mut res: Vec<Author> = Vec::new();
    for commit in commits {
        let name = commit.author().name().unwrap().to_string();
        if !names.contains(&name) {
            res.push(Author::new(name.as_str(), HashMap::new()));
        }
        names.push(name);
    }
    res
}

fn update_occurrence<'a>(word: &'a str, map: &mut HashMap<&'a str, usize>) {
    map.entry(word).and_modify(|i| *i += 1);
}

fn naughty_word(word: &str, naughty_list: &[&str]) -> bool {
    naughty_list.contains(&word)
}

fn filter_occurrences(map: &mut HashMap<&str, usize>) {
    map.retain(|_, val| val > &mut 0);
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
    fn test_update_occurrences() {
        let curses: Vec<&str> = CURSES.lines().collect();
        let mut occurrences: HashMap<&str, usize> = HashMap::new();
        for curse in &curses {
            occurrences.entry(curse).or_insert(0);
        }
        update_occurrence("boobs", &mut occurrences);
        update_occurrence("crap", &mut occurrences);
        update_occurrence("boobs", &mut occurrences);

        assert_eq!(2, occurrences.remove("boobs").unwrap());
        assert_eq!(1, occurrences.remove("crap").unwrap());
    }

    #[test]
    fn test_filter_occurrences() {
        let curses: Vec<&str> = CURSES.lines().collect();
        let mut occurrences: HashMap<&str, usize> = HashMap::new();
        let actual: HashMap<&str, usize> =
            [("shite", 1), ("goddamn", 1), ("shit", 1), ("fucking", 1)]
                .iter()
                .cloned()
                .collect();
        for curse in &curses {
            occurrences.entry(curse).or_insert(0);
        }
        for word in "this is a fucking shit sentence with no goddamn shite in it".split_whitespace()
        {
            if naughty_word(&word, &curses) {
                update_occurrence(word, &mut occurrences);
            }
        }
        filter_occurrences(&mut occurrences);
        assert_eq!(actual, occurrences);
    }
}
