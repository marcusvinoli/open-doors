pub mod error;

use std::ffi::OsStr;
use std::fmt::Display;
use std::process::{Command, Output};

pub use error::GitError as GitError;

fn git_run_cmd<T: AsRef<OsStr>, U: IntoIterator<Item = T>>(args: U) -> std::io::Result<Output> {
    Command::new("git")
        .args(args)
        .output()
}

pub fn is_git_installed() -> Result<bool, GitError> {
    Ok(git_run_cmd(["-v"])?.status.success())
}

pub fn clone<T: AsRef<str> + Display, U: AsRef<str> + Display>(remote: T, path: U) -> Result<bool, GitError> {
    Ok(git_run_cmd(["clone", &remote.to_string(), &path.to_string()])?.status.success())
}

pub fn add_all<T: AsRef<str> + Display>(path: T) -> Result<bool, GitError> {
    add(path, "-A")
}

pub fn add<T: AsRef<str> + Display, U: AsRef<str> + Display>(path: T, doc: U) -> Result<bool, GitError> {
    Ok(git_run_cmd(["-C", &path.to_string(), "add", &doc.to_string()])?.status.success())
}

pub fn get_user_name<T: AsRef<str> + Display>(path: T) -> Result<String, GitError> {
    let res = git_run_cmd(["-C", &path.to_string(), "user.name"])?;
    return Ok(String::from_utf8(res.stdout).unwrap_or_default());
}

pub fn get_user_email<T: AsRef<str> + Display>(path: T) -> Result<String, GitError> {
    let res = git_run_cmd(["-C", &path.to_string(), "user.email"])?;
    return Ok(String::from_utf8(res.stdout).unwrap_or_default());
}

pub fn set_user_name<T: AsRef<str> + Display, U: AsRef<str> + Display>(path: T, name: U) -> Result<String, GitError> {
    git_run_cmd(["-C", &path.to_string(), "user.name", &name.to_string()])?;
    get_user_name(path)
}

pub fn set_user_email<T: AsRef<str> + Display, U: AsRef<str> + Display>(path: T, email: U) -> Result<String, GitError> {
    git_run_cmd(["-C", &path.to_string(), "user.email", &email.to_string()])?;
    get_user_email(path)
}

pub fn commit<T: AsRef<str> + Display, U: AsRef<str> + Display>(path: T, message: U) -> Result<bool, GitError> {
    Ok(git_run_cmd(["-C", &path.to_string(), "commit", "-m", &message.to_string()])?.status.success())
}

pub fn is_git_repository<T: AsRef<str> + Display>(path: T) -> Result<bool, GitError> {
    Ok(git_run_cmd(["-C", &path.to_string(), "status"])?.status.success())
}

pub fn init<T: AsRef<str> + Display>(path: T) -> Result<bool, GitError> {
    Ok(git_run_cmd(["-C", &path.to_string(), "init"])?.status.success())
}
