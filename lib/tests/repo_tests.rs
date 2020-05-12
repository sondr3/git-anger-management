use git2::Repository;
use git_anger_library::repo::Repo;
use std::path::Path;

#[test]
fn test_commit_count() {
    let repo = Repository::open(Path::new("./tests/repo").to_path_buf()).unwrap();
    let commits = Repo::commits(&repo).unwrap();
    assert_eq!(commits.len(), 5);
}

#[test]
fn test_example_repo() {
    let repo = Repo::new(Path::new("./tests/repo")).unwrap();

    assert_eq!(repo.authors.len(), 3);
    assert_eq!(repo.total_commits, 5);
    assert_eq!(repo.total_curses, 7);

    let john = repo.authors.get("John Doe").unwrap();
    assert_eq!(john.total_curses, 2);
    assert_eq!(john.total_commits, 1);
    assert_eq!(john.curses.get("bloody").unwrap(), &1);
    assert_eq!(john.curses.get("damn").unwrap(), &1);

    let me = repo.authors.get("Sondre Nilsen").unwrap();
    assert_eq!(me.total_curses, 3);
    assert_eq!(me.total_commits, 3);
    assert_eq!(me.curses.get("fuck").unwrap(), &1);
    assert_eq!(me.curses.get("fucking").unwrap(), &1);
    assert_eq!(me.curses.get("shitty").unwrap(), &1);

    let ola = repo.authors.get("Ola Nordmann").unwrap();
    assert_eq!(ola.total_curses, 2);
    assert_eq!(ola.total_commits, 1);
    assert_eq!(ola.curses.get("fucking").unwrap(), &1);
    assert_eq!(ola.curses.get("bitch").unwrap(), &1);
}

#[test]
fn test_curse_total() {
    let repo = Repo::new(Path::new("./tests/repo")).unwrap();
    let total: usize = repo
        .authors
        .values()
        .map(|a| a.curses.values().sum::<usize>())
        .sum();

    assert_eq!(total, repo.total_curses);
}
