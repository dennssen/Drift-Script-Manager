use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub fn has_git() -> bool {
    Command::new("git")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn create_local_repo(project_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .arg("init")
        .current_dir(&project_path)
        .output()?;

    if !output.status.success() {
        return Err(format!("Failed to initialize git repo: {}", String::from_utf8_lossy(&output.stderr)).into())
    }

    create_git_ignore(&project_path)?;

    let output = Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(&project_path)
        .output()?;

    if !output.status.success() {
        return Err(format!("Failed to add files: {}", String::from_utf8_lossy(&output.stderr)).into())
    }

    let output = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Initial project setup")
        .current_dir(&project_path)
        .output()?;

    if !output.status.success() {
        return Err(format!("Failed to commit: {}", String::from_utf8_lossy(&output.stderr)).into())
    }

    Ok(())
}

fn create_git_ignore(repo_path: &PathBuf) -> io::Result<()> {
    let git_ignore_path = repo_path.join(".gitignore");
    let mut git_ignore_file = File::create(git_ignore_path)?;

    git_ignore_file.write_all(b"/Builds")?;

    Ok(())
}