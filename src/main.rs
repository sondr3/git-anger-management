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
    let mut occurrences: HashMap<String, usize> = HashMap::new();
    for curse in &curses {
        occurrences.entry(curse.to_string()).or_insert(0);
    }
    for word in "this is a fucking shit sentence with no goddamn shite in it".split(' ') {
        if curses.contains(&word) {
            occurrences.entry(word.to_string()).and_modify(|i| *i += 1);
        }
    }
    println!("{:?}", repo.workdir());
    let occurrences: HashMap<String, usize> = occurrences
        .into_iter()
        .filter(|&(_, val)| val > 0)
        .collect();
    println!("{:#?}", occurrences);
    Ok(())
}
