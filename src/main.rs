#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![forbid(unsafe_code)]
extern crate git2;

use git2::Repository;
use std::collections::HashMap;
use std::env;

static CURSES: &str = include_str!("words.txt");

//struct Author {
//    name: String,
//    curses: HashMap<String, usize>,
//}

fn main() -> Result<(), Box<std::error::Error>> {
    let curses: Vec<&str> = CURSES.lines().collect();
    let path = env::current_dir()?;
    let repo = Repository::open(path)?;
    let mut occurrences: HashMap<&str, usize> = HashMap::new();
    for curse in &curses {
        occurrences.entry(curse).or_insert(0);
    }
    println!("{:?}", repo.workdir());
    filter_occurrences(&mut occurrences);
    println!("{:#?}", occurrences);
    Ok(())
}

fn naughty_word(word: &str, naughty_list: &[&str]) -> bool {
    if naughty_list.contains(&word) {
        return true;
    }
    false
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
                occurrences.entry(word).and_modify(|i| *i += 1);
            }
        }
        filter_occurrences(&mut occurrences);
        assert_eq!(actual, occurrences);
    }
}
