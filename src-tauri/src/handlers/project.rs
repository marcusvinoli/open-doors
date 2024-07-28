use std::path::PathBuf;
use tauri::command; 

use crate::core::{error::OpenDoorsError, project::{Project, ProjectManifest}};

#[command] 
pub fn create_project(path: PathBuf, man: ProjectManifest) -> Result<Project, OpenDoorsError> {
    Ok(Project::create(&path, &man)?)
}

#[command]
pub fn read_project(path: PathBuf) -> Result<Project, OpenDoorsError> {
  Ok(Project::read(&path)?)
}

#[command]
pub fn update_project(path: PathBuf, man: ProjectManifest) -> Result<Project, OpenDoorsError> {
  Project::update_manifest(&path, &man)?;
  Ok(Project::read(&path)?)
}

#[command]
pub fn delete_project(path: PathBuf) -> Result<(), OpenDoorsError> {
  Ok(Project::delete(&path)?)
}
