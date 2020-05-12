use crate::{
    author::Author,
    core::{naughty_word, split_into_clean_words},
};
use git2::{Commit, Repository};
use std::io::Write;
use std::{collections::HashMap, env, error::Error, io, path::Path};
use tabwriter::TabWriter;

/// A simple representation of a git repository.
#[derive(Debug)]
pub struct Repo {
    /// Name of the repository.
    pub name: String,
    /// Count of the total amount of commits in the repository.
    pub total_commits: usize,
    /// Count of the total amount of curses used in the commits.
    pub total_curses: usize,
    /// HashMap of all the naughty words used by the authors.
    pub curses: HashMap<String, usize>,
    /// HashMap of all the authors that have been committed.
    pub authors: HashMap<String, Author>,
}

impl Repo {
    /// Creates a new and empty repository.
    pub fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        let repo = Repository::open(path)?;
        let commits = Repo::commits(&repo)?;

        let repo = match path.file_name() {
            Some(path) => path.to_str().unwrap().to_owned(),
            None => env::current_dir()?.to_str().unwrap().to_owned(),
        };

        let mut repo = Repo {
            name: repo,
            total_commits: 0,
            total_curses: 0,
            curses: HashMap::new(),
            authors: HashMap::new(),
        };

        repo.build(commits);
        repo.count_curses();

        Ok(repo)
    }

    /// Checks if an author exists and creates a new author if she/he doesn't
    /// exist.
    pub fn author(&mut self, author_name: &str) -> &mut Author {
        if !self.authors.contains_key(author_name) {
            self.authors
                .entry(author_name.into())
                .or_insert_with(|| Author::new(author_name));
        }

        self.authors.get_mut(author_name).expect("exists")
    }

    /// Counts all the naughty words used by authors.
    pub fn count_curses(&mut self) {
        for author in self.authors.values() {
            for (name, curse) in &author.curses {
                self.curses
                    .entry(name.to_string())
                    .and_modify(|c| *c += *curse)
                    .or_insert_with(|| *curse);
            }
        }
    }

    /// Count total naughty authors in repository.
    fn total_naughty_authors(&self) -> usize {
        self.authors.values().filter(|a| a.is_naughty()).count()
    }

    /// Build a table to display naughty authors and their words.
    pub fn print_list(&self) -> Result<(), Box<dyn Error>> {
        let mut tw = TabWriter::new(vec![]);
        let curses = Repo::sort(&self.curses);

        self.table_headers(&mut tw, &curses)?;
        self.table_separators(&mut tw, &curses)?;
        self.table_authors(&mut tw, &curses)?;

        if self.total_naughty_authors() > 1 {
            self.table_separators(&mut tw, &curses)?;
            self.table_total(&mut tw, &curses)?;
        }

        tw.flush()?;

        write!(io::stdout(), "{}", String::from_utf8(tw.into_inner()?)?)?;
        io::stdout().flush()?;

        Ok(())
    }

    /// Create a sorted `Vec` from a HashMap of curses, sorted by counts
    fn sort(curses: &HashMap<String, usize>) -> Vec<(String, usize)> {
        let mut curses: Vec<(&String, &usize)> = curses.iter().collect();
        curses.sort_by(|(a, _), (b, _)| a.cmp(b));
        let curses: Vec<_> = curses
            .iter()
            .map(|(c, i)| ((*c).to_string(), **i))
            .collect();
        curses
    }

    /// Add headers to a table
    fn table_headers(
        &self,
        tw: &mut TabWriter<Vec<u8>>,
        curses: &[(String, usize)],
    ) -> Result<(), Box<dyn Error>> {
        let mut header = String::new();
        header.push_str("Author");
        header.push_str("\t");

        curses
            .iter()
            .for_each(|(curse, _)| header.push_str(&[curse, "\t"].concat()));

        header.push_str(&["Total", "\t"].concat());

        writeln!(tw, "{}", header)?;

        Ok(())
    }

    fn table_separators(
        &self,
        tw: &mut TabWriter<Vec<u8>>,
        curses: &[(String, usize)],
    ) -> Result<(), Box<dyn Error>> {
        let mut sep = String::new();
        sep.push_str(&[&"-".repeat("Author".len()), "\t"].concat());

        curses
            .iter()
            .map(|(curse, _)| (curse, curse.len()))
            .for_each(|(_, curse_len)| sep.push_str(&[&"-".repeat(curse_len), "\t"].concat()));

        sep.push_str(&[&"-".repeat("Total".len()), "\t"].concat());

        writeln!(tw, "{}", sep)?;
        Ok(())
    }

    fn table_authors(
        &self,
        tw: &mut TabWriter<Vec<u8>>,
        curses: &[(String, usize)],
    ) -> Result<(), Box<dyn Error>> {
        let mut authors: Vec<_> = self.authors.values().collect();
        authors.sort_unstable_by_key(|a| &a.name);

        for author in authors {
            if author.is_naughty() {
                let mut out = String::new();
                out.push_str(&[&author.name, "\t"].concat());
                // FIXME: use authors curses, not global curses

                for (curse, _) in curses {
                    if let Some(count) = author.curses.get(curse) {
                        out.push_str(&[&count.to_string(), "\t"].concat());
                    } else {
                        out.push_str("0\t");
                    }
                }
                out.push_str(&author.curses.values().sum::<usize>().to_string());

                writeln!(tw, "{}", out)?;
            }
        }

        Ok(())
    }

    fn table_total(
        &self,
        tw: &mut TabWriter<Vec<u8>>,
        curses: &[(String, usize)],
    ) -> Result<(), Box<dyn Error>> {
        let mut out = String::new();

        out.push_str(&["Overall", "\t"].concat());

        curses
            .iter()
            .for_each(|(_, count)| out.push_str(&[&count.to_string(), "\t"].concat()));

        out.push_str(&self.total_curses.to_string());

        writeln!(tw, "{}", out)?;

        Ok(())
    }

    /// Build a list of commits by walking the history of a repository.
    pub fn commits(repo: &Repository) -> Result<Vec<Commit>, Box<dyn Error>> {
        let mut revwalk = repo.revwalk()?;
        let mut commits: Vec<Commit> = Vec::new();
        revwalk.push_head()?;
        for commit_id in revwalk {
            let commit = repo.find_commit(commit_id?)?;
            commits.push(commit);
        }

        Ok(commits)
    }

    /// Documentation dammit!
    pub fn build(&mut self, commits: Vec<Commit>) {
        for commit in &commits {
            if let (Some(author_name), Some(commit_message)) = (
                commit.author().name(),
                commit.message().map(|w| w.to_lowercase()),
            ) {
                let mut curses_added = 0;
                {
                    let author = self.author(author_name);
                    author.total_commits += 1;
                    for word in split_into_clean_words(&commit_message) {
                        if naughty_word(word) {
                            author.total_curses += 1;
                            curses_added += 1;
                            author.update_occurrence(word);
                        }
                    }
                }
                self.total_commits += 1;
                self.total_curses += curses_added;
            } else {
                eprintln!(
                    "Skipping commit {:?} because either the commit author or message is missing",
                    commit
                );
            }
        }
    }
}
