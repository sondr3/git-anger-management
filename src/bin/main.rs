use console::Term;
use git_anger_management::repo::Repo;
use std::{env, error::Error, path::PathBuf, time::Instant};
use structopt::{clap::AppSettings, StructOpt};

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
    let json = opt.json;

    let repo = Repo::new(&path)?;
    let term = Term::stderr();

    term.write_line("Crunching commits...")?;
    term.clear_last_lines(1)?;
    if verbose {
        println!("Took {:?} to parse {}", start.elapsed(), repo.name);
    }

    if json {
        repo.print_json()?;
    } else {
        repo.print_list()?;
    }

    Ok(())
}
