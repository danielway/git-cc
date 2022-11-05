use std::{
    io::Error,
    process::{Command, Output},
};

/// Perform a commit with the specified messages. Does not validate the repo's status.
pub fn commit(message: &str) -> Result<Output, Error> {
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()
}
