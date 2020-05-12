use console::Term;
use git2::Repository;
use git_anger_management::Repo;
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
    #[structopt(short, long)]
    /// Print output as JSON instead of a prettified table
    json: bool,
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
    let commits = Repo::commits(&repo)?;

    let mut repo = Repo::new(match path.file_name() {
        Some(path) => path.to_str().unwrap().to_owned(),
        None => env::current_dir()?.to_str().unwrap().to_owned(),
    });

    let term = Term::stderr();

    term.write_line("Crunching commits...")?;
    repo.build(commits);

    term.clear_last_lines(1)?;
    repo.count_curses();
    if verbose {
        println!("Took {:?} to parse {}", start.elapsed(), repo.name);
    }

    print!("{}", repo);

    Ok(())
}
