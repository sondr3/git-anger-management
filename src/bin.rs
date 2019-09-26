use git2::{Commit, Repository};
use git_anger_management::{naughty_word, split_into_clean_words, Repo};
use indicatif::{ProgressBar, ProgressStyle};
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
    /// Disable the progress bar
    progress: bool,
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
    let progress = opt.progress;

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

    let mut progress_bar = ProgressBar::hidden();
    if !progress {
        progress_bar = ProgressBar::new(commits.len() as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed}] ▕{bar:40}▏{msg} ({eta})")
                .progress_chars("█▉▊▋▌▍▎▏ "),
        );
    }

    let mut repo = Repo::new(match path.file_name() {
        Some(path) => path.to_str().unwrap().to_owned(),
        None => env::current_dir()?.to_str().unwrap().to_owned(),
    });

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
            progress_bar.set_message(&format!("commit {:?}", commit.id()));
            progress_bar.inc(1);
        } else {
            eprintln!(
                "Skipping commit {:?} because either the commit author or message is missing",
                commit
            );
        }
    }

    progress_bar.finish_and_clear();
    repo.count_curses();
    if verbose {
        println!("Took {:?} to parse {}", start.elapsed(), repo.name);
    }

    println!("{}", repo);
    if !opt.repo {
        for author in repo.authors.values() {
            if author.is_naughty() {
                println!("{}", author);
            }
        }
    }

    Ok(())
}
