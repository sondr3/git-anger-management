use console::Term;
use git2::{Commit, Repository};
use git_anger_management::{naughty_word, split_into_clean_words, Repo};
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::time::Instant;
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "git anger-management",
    about = "Ever wondered how angry your commits are? Look no further...",
    global_settings(&[AppSettings::ColoredHelp])
)]
struct Cli {
    #[structopt(short, long)]
    /// Verbose output
    verbose: bool,
    #[structopt(short, long)]
    /// Only display information about repo
    repo: bool,
    #[structopt(parse(from_os_str))]
    /// Directory to parse commits from
    directory: Option<PathBuf>,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
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

    let mut repo = Repo::new(match path.file_name() {
        Some(path) => path.to_str().unwrap().to_owned(),
        None => env::current_dir()?.to_str().unwrap().to_owned(),
    });

    let term = Term::stderr();

    term.write_line("Crunching commits...")?;
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
                    if naughty_word(word) {
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

    term.clear_last_lines(1)?;
    repo.count_curses();
    if verbose {
        println!("Took {:?} to parse {}", start.elapsed(), repo.name);
    }

    print!("{}", repo);

    Ok(())
}
