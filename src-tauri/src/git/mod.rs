pub mod error;

use std::ffi::OsStr;
use std::fmt::Display;
use std::process::{Command, Output};
pub use error::GitError as GitError;

fn git_run_cmd<T: AsRef<OsStr>, U: IntoIterator<Item = T>>(args: U) -> std::io::Result<Output> {
    let mut cmd = Command::new("git");
    cmd.args(args);

    #[cfg(target_os = "windows")] {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    cmd.output()
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
    let res = git_run_cmd(["-C", &path.to_string(), "config", "user.name"])?;
    let ret = String::from_utf8(res.stdout)?.replace("\n", "").replace("\r", "");
    return Ok(ret);
}

pub fn get_user_email<T: AsRef<str> + Display>(path: T) -> Result<String, GitError> {
    let res = git_run_cmd(["-C", &path.to_string(), "config", "user.email"])?;
    let ret = String::from_utf8(res.stdout)?.replace("\n", "").replace("\r", "");
    return Ok(ret);
}

pub fn set_user_name<T: AsRef<str> + Display, U: AsRef<str> + Display>(path: T, name: U) -> Result<String, GitError> {
    git_run_cmd(["-C", &path.to_string(), "config", "user.name", &name.to_string()])?;
    get_user_name(path)
}

pub fn set_user_email<T: AsRef<str> + Display, U: AsRef<str> + Display>(path: T, email: U) -> Result<String, GitError> {
    git_run_cmd(["-C", &path.to_string(), "config", "user.email", &email.to_string()])?;
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

pub fn set_remote<T: AsRef<str> + Display>(path: T, remote: &String) -> Result<bool, GitError> {
    Ok(git_run_cmd(["-C", &path.to_string(), "remote", "add", "origin", remote])?.status.success())
}

pub fn set_pull_strategy<T: AsRef<str> + Display>(path: T) -> Result<bool, GitError> {
    Ok(git_run_cmd(["-C", &path.to_string(), "config", "pull.rebase", "true"])?.status.success())
}

pub fn set_push_strategy<T: AsRef<str> + Display>(path: T) -> Result<bool, GitError> {
    Ok(git_run_cmd(["-C", &path.to_string(), "push", "--set-upstream", "origin", "master"])?.status.success())
}

pub fn first_pull<T: AsRef<str> + Display>(path: T) -> Result<bool, GitError> {
    Ok(git_run_cmd(["-C", &path.to_string(), "pull", "origin", "main"])?.status.success())
}

pub fn pull<T: AsRef<str> + Display>(path: T) -> Result<bool, GitError> {
    Ok(git_run_cmd(["-C", &path.to_string(), "pull"])?.status.success())
}

pub fn push<T: AsRef<str> + Display>(path: T) -> Result<bool, GitError> {
    Ok(git_run_cmd(["-C", &path.to_string(), "push"])?.status.success())
}

pub fn create_tag<T: AsRef<str> + Display>(path: T, tag: &String) -> Result<bool, GitError> {
    Ok(git_run_cmd(["-C", &path.to_string(), "tag", tag])?.status.success())
}

pub fn read_tags<T: AsRef<str> + Display>(path: T) -> Result<bool, GitError> {
    Ok(git_run_cmd(["-C", &path.to_string(), "tag", "-v"])?.status.success())
}
