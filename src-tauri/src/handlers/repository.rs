use std::{path::PathBuf, sync::Mutex};
use git2::Repository as GitRepository;
use tauri::{command, State};

use crate::core::{error::OpenDoorsError, repository::Repository};

#[command]
pub fn clone_repository(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf, remote: String) -> Result<Repository, OpenDoorsError> {
	GitRepository::clone(&remote, &path)?;
	read_repository(state, path)
}

#[command]
pub fn read_repository(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf) -> Result<Repository, OpenDoorsError> {
	let git_repo: GitRepository = GitRepository::open(&path)?;
	let repo: Repository = Repository::read(&path)?;
	*state.lock().unwrap() = Some(git_repo);
	Ok(repo)
}

#[command]
pub fn create_repository(state: State<'_, Mutex<Option<GitRepository>>>, path: &str, name: &str, remote: Option<String>) -> Result<Repository, OpenDoorsError> {
	let mut repo = state.lock().unwrap();
	Ok(Repository::create(&mut repo, &path, &name, &remote)?)
}
