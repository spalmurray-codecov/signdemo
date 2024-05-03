use std::{path::Path, process::{Command, Output} };

pub fn sign_file(file: &str) -> bool {
    if file.is_empty() {
        eprintln!("Invalid usage.\nExample usage: signdemo file");
        return false;
    }

    if !Path::new(file).exists() {
        eprintln!("That file does not exist. Check input path and try again.");
        return false;
    }

    let result: Option<Output> = Command::new("gpg").args(["-ba", file]).output().ok();
    dbg!(&result);

    if result.is_some() && !result.unwrap().status.success() {
        eprintln!("Something went wrong while signing the file. Check gpg config and try again.");
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_arg() {
        assert!(!sign_file(""));
    }

    #[test]
    fn file_does_not_exist() {
        assert!(!sign_file("a"));
    }

    #[test]
    fn gpg_error() {
        assert!(!sign_file(".gitignore"));
    }
}
