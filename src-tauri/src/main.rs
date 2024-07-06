#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod core;
pub mod git;
mod state;

use std::{path::PathBuf, sync::{Arc, Mutex}};

use core::{OpenDoorsError, Repository, RespositoryManifest};
use crate::state::State;

#[derive(Default)]
struct AppState(Arc<Mutex<State>>);

fn main() {
	tauri::Builder::default()
		.manage(AppState::default())
		.invoke_handler(tauri::generate_handler![
		clone_repo,
		read_repo,
		create_repo,
		])
		.run(tauri::generate_context!())
		.expect("Error while running OpenDOORs.");
}

#[tauri::command]
fn clone_repo(st: tauri::State<AppState>, repo: RespositoryManifest) -> Result<Repository, OpenDoorsError> {
	let rep_man = repo.clone();
	git::clone(rep_man.remote.unwrap_or_default().clone(), rep_man.path.clone().display().to_string().clone())?;
	read_repo(st, repo.path)
}

#[tauri::command]
fn read_repo(st: tauri::State<AppState>, path: PathBuf) -> Result<Repository, OpenDoorsError> {
	let repo: Repository = Repository::read(&path)?;
	let repo_res = repo.clone();
	st.0.lock().unwrap().repository = repo;
	Ok(repo_res)
}

#[tauri::command]
fn create_repo(st: tauri::State<AppState>, repo: RespositoryManifest) -> Result<Repository, OpenDoorsError> {
	let repo: Repository = Repository::create(&repo.path, repo.name, repo.remote)?;
	read_repo(st, repo.manifest.path)
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
