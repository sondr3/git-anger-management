extern crate git2;

use git2::Repository;
use std::env;

fn main() -> Result<(), Box<std::error::Error>> {
    let curses = vec!["arse", "ass", "asshole", "bitch", "balls", "bollock", "bugger", "cock", "cunt", "dick", "fuck", "goddamn", "damn", "shit", "crap", "piss", "shit", "shite"];
    let path = env::current_dir()?;
    let repo = Repository::open(path)?;
    println!("{:?}", curses);
    println!("{:?}", repo.workdir());
    Ok(())
}
