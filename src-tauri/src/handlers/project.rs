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
