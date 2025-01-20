use std::{path::PathBuf, sync::Mutex};
use git2::Repository;
use tauri::{command, State}; 

use crate::core::{error::OpenDoorsError, project::{Project, ProjectManifest}};

#[command] 
pub fn create_project(state: State<'_, Mutex<Option<Repository>>>, path: &str, man: ProjectManifest) -> Result<Project, OpenDoorsError> {
	let repo = state.lock().unwrap();
	Ok(Project::create(&repo, &path, &man)?)
}

#[command]
pub fn read_project(path: PathBuf) -> Result<Project, OpenDoorsError> {
	Ok(Project::read(&path)?)
}

#[command]
pub fn update_project(state: State<'_, Mutex<Option<Repository>>>, path: PathBuf, man: ProjectManifest) -> Result<Project, OpenDoorsError> {
	let repo = state.lock().unwrap();
	Project::update_manifest(&repo, &path, &man)?;
	Ok(Project::read(&path)?)
}

#[command]
pub fn delete_project(state: State<'_, Mutex<Option<Repository>>>, path: PathBuf) -> Result<(), OpenDoorsError> {
	let repo = state.lock().unwrap();
	Ok(Project::delete(&repo, &path)?)
}
