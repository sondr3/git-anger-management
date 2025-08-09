use std::{collections::HashMap, error::Error, io, io::Write};

use git_anger_management::repo::Repo;
use tabwriter::TabWriter;

/// Serialize the `Repo` struct into a JSON-object and print it.
pub fn print_json(repo: &Repo) -> Result<(), Box<dyn Error>> {
    let serialized = serde_json::to_string(&repo)?;
    write!(io::stdout(), "{}", serialized)?;
    io::stdout().flush()?;

    Ok(())
}

/// Build a table to display naughty authors and their words.
pub fn print_table(repo: &Repo) -> Result<(), Box<dyn Error>> {
    let mut tw = TabWriter::new(vec![]);
    let curses = sort(&repo.curses);

    table_headers(&mut tw, &curses)?;
    table_separators(&mut tw, &curses)?;
    table_authors(repo, &mut tw, &curses)?;

    if repo.total_naughty_authors() > 1 {
        table_separators(&mut tw, &curses)?;
        table_total(repo, &mut tw, &curses)?;
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
    tw: &mut TabWriter<Vec<u8>>,
    curses: &[(String, usize)],
) -> Result<(), Box<dyn Error>> {
    let mut header = String::new();
    header.push_str("Author");
    header.push('\t');

    curses
        .iter()
        .for_each(|(curse, _)| header.push_str(&[curse, "\t"].concat()));

    header.push_str(&["Total", "\t"].concat());

    writeln!(tw, "{}", header)?;

    Ok(())
}

/// Add separators (`----`) to a table based on word lengths.
fn table_separators(
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

/// Add all the naughty authors to the table.
fn table_authors(
    repo: &Repo,
    tw: &mut TabWriter<Vec<u8>>,
    curses: &[(String, usize)],
) -> Result<(), Box<dyn Error>> {
    let mut authors: Vec<_> = repo.authors.values().collect();
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

/// Sum up the total naughty count and print it.
fn table_total(
    repo: &Repo,
    tw: &mut TabWriter<Vec<u8>>,
    curses: &[(String, usize)],
) -> Result<(), Box<dyn Error>> {
    let mut out = String::new();

    out.push_str(&["Overall", "\t"].concat());

    curses
        .iter()
        .for_each(|(_, count)| out.push_str(&[&count.to_string(), "\t"].concat()));

    out.push_str(&repo.total_curses.to_string());

    writeln!(tw, "{}", out)?;

    Ok(())
}
