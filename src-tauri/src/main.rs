#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod core;
pub mod git;
mod state;

use std::path::PathBuf;

use core::{OpenDoorsError, Project, Repository, RepositoryManifest, ProjectManifest};

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
		clone_repo,
		read_repo,
		create_repo,
		])
		.run(tauri::generate_context!())
		.expect("Error while running OpenDOORs.");
}

#[tauri::command]
fn clone_repo(man: RepositoryManifest, path: PathBuf) -> Result<Repository, OpenDoorsError> {
	git::clone(man.remote.unwrap_or_default().clone(), path.clone().display().to_string().clone())?;
	read_repo(path)
}

#[tauri::command]
fn read_repo(path: PathBuf) -> Result<Repository, OpenDoorsError> {
	Ok(Repository::read(path)?)
}

#[tauri::command]
fn create_repo(man: RepositoryManifest, path: PathBuf) -> Result<Repository, OpenDoorsError> {
	Ok(Repository::create(path, man.name, man.remote)?)
}

#[tauri::command]
fn create_project(project: ProjectManifest, path: PathBuf) -> Result<Project, OpenDoorsError> {
	todo!()
}

/*
// REPOSITORY
fn update_repo() {}
fn delete_repo() {}

// PROJECTS
fn read_project() {}
fn create_project() {}
fn delete_project() {}
fn update_project() {}

// FOLDER
fn read_folder() {}
fn create_folder() {}
fn delete_folder() {}
fn update_folder() {}

// MODULE
fn read_module() {}
fn create_module() {}
fn update_module() {}
fn delete_module() {}

// OBJECTS
fn read_object() {}
fn create_object() {}
fn update_object() {}
fn delete_object() {}

*/
