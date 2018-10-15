#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate lazy_static;

extern crate crossbeam_channel;
extern crate git2;
extern crate rayon;

use git2::Repository;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;
use std::{char, env, fmt};
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

    fn update_occurrences(&mut self, curse: &str, occurrences: usize) {
        self.curses
            .get_mut(curse)
            .map(|count| *count += occurrences)
            .unwrap_or_else(|| {
                self.curses.insert(curse.to_owned(), occurrences);
            })
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

    let (s, r) = crossbeam_channel::unbounded();

    let mut repo = Repo::new(repo_path.file_name().unwrap().to_str().unwrap());
    for commit in &commits {
        if let (Some(author_name), Some(commit_message)) = (
            commit.author().name().map(|n| n.to_owned()),
            commit.message().map(|msg| msg.to_lowercase()),
        ) {
            let job_sender = s.clone();
            rayon::spawn(move || {
                let mut naughty_words = HashMap::new();
                for word in split_into_clean_words(&commit_message).filter(|w| naughty_word(w)) {
                    naughty_words
                        .get_mut(word)
                        .map(|count| *count += 1)
                        .unwrap_or_else(|| {
                            naughty_words.insert(word.to_owned(), 1);
                        })
                }
                job_sender.send((author_name, naughty_words));
            })
        } else {
            println!(
                "Warning: skipping commit {:?} because author name OR commit message missing",
                commit
            );
        }
    }

    // Drop the sender here so that when the work is done the channel closes
    drop(s);

    while let Some((author_name, naughty_words)) = r.recv() {
        let mut total_curses_added = 0;

        {
            let author = repo.author_for(&author_name);
            author.total_commits += 1;
            for (word, occurrences) in naughty_words {
                author.total_curses += occurrences;
                total_curses_added += occurrences;
                author.update_occurrences(&word, occurrences);
            }
        }

        repo.total_commits += 1;
        repo.total_curses += total_curses_added;
    }

    println!("{}", repo);
    for mut author in repo.authors.values() {
        if author.is_naughty() {
            println!("{}", author);
        }
    }
    Ok(())
}

fn split_into_clean_words(l: &str) -> impl Iterator<Item = &str> {
    l.split(|c| !char::is_alphabetic(c))
        .filter(|w| !w.is_empty())
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
        let clean_words = split_into_clean_words("This! is a string: with, some. words in? it;")
            .collect::<Vec<_>>();
        assert_eq!(
            clean_words,
            vec!["This", "is", "a", "string", "with", "some", "words", "in", "it"]
        );
    }

    // The new implementation yields str slices and does not allocate, which
    // is good. However, it differs from the previous implementation because
    // words with invalid characters in the middle are not joined together.
    #[test]
    fn does_not_join_words_with_invalid_characters_in_middle() {
        let clean_words = split_into_clean_words("inv!alid").collect::<Vec<_>>();
        assert_eq!(clean_words, vec!["inv", "alid"]);
    }
}
