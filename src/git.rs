use std::{
    io::Error,
    process::{Command, Output},
    str::from_utf8,
};

/// Determines whether this repo has any staged changes.
pub fn has_staged_changes() -> Result<bool, Error> {
    let output = Command::new("git")
        .arg("status")
        .arg("-s")
        .arg("--untracked-files=no")
        .output()?;

    let status = from_utf8(&output.stdout).expect("status should be available");

    let staged_changes = status.lines().filter(|line| {
        let index_status = line.chars().next();
        index_status.is_some() && index_status.unwrap() != ' '
    });

    Ok(staged_changes.count() >= 1)
}

/// Perform a commit with the specified messages. Does not validate the repo's status.
pub fn commit(message: &str) -> Result<Output, Error> {
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()
}
