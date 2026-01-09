use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Write};
use std::path::PathBuf;
use std::sync::OnceLock;
use git2::Repository;
use which::which;

static HAS_GIT: OnceLock<bool> = OnceLock::new();

pub fn has_git() -> bool {
    *HAS_GIT.get_or_init(|| {
        which("git").is_ok()
    })
}

pub fn create_local_repo(project_path: &PathBuf) -> io::Result<()> {
    let repo = Repository::init(project_path)
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    create_git_ignore(project_path)?;

    let mut index = repo.index()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    index.write()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    let tree_id = index.write_tree()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    let tree = repo.find_tree(tree_id)
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    let sig = repo.signature()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        "Initial project setup",
        &tree,
        &[]
    ).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;

    Ok(())
}

fn create_git_ignore(repo_path: &PathBuf) -> io::Result<()> {
    let git_ignore_path = repo_path.join(".gitignore");
    let mut git_ignore_file = File::create(git_ignore_path)?;

    git_ignore_file.write_all(b"/Builds")?;

    Ok(())
}