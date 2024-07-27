use std::path::PathBuf;
use tauri::command;

use crate::core::{error::OpenDoorsError, repository::Repository};
use crate::git;

#[command]
pub fn clone_repo(path: PathBuf, remote: String) -> Result<Repository, OpenDoorsError> {
	git::clone(&remote, &path.display().to_string())?;
	read_repo(path)
}

#[command]
pub fn read_repo(path: PathBuf) -> Result<Repository, OpenDoorsError> {
    let repo = Repository::read(&path)?;
    Ok(repo)
}

#[command]
pub fn create_repo(path: PathBuf, name: String, remote: Option<String>) -> Result<Repository, OpenDoorsError> {
	Ok(Repository::create(&path, &name, &remote)?)
}
