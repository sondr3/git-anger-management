use git2::Repository;
use git_anger_management::repo::Repo;
use std::path::{Path, PathBuf};

#[test]
fn test_commit_count() {
    let repo = Repository::open(Path::new("./tests/repo").to_path_buf()).unwrap();
    let commits = Repo::commits(&repo).unwrap();
    assert_eq!(commits.len(), 4);
}

#[test]
fn test_example_repo() {
    let repo = Repository::open(Path::new("./tests/repo").to_path_buf()).unwrap();
    let commits = Repo::commits(&repo).unwrap();
    let mut repo = Repo::new("repo");
    repo.build(commits);

    assert_eq!(repo.authors.len(), 2);
    assert_eq!(repo.total_commits, 4);
    assert_eq!(repo.total_curses, 6);

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
}
