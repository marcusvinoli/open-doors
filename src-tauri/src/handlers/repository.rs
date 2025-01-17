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
pub fn create_repository(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf, name: String, remote: Option<String>) -> Result<Repository, OpenDoorsError> {
	let git_repo: GitRepository = GitRepository::init(&path)?;
	let url: Option<String> = remote.clone();
	if let Some(url) = url {
		git_repo.remote_set_url("origin", &url)?;
	}
	*state.lock().unwrap() = Some(git_repo);
	Ok(Repository::create(&path, &name, &remote)?)
}
