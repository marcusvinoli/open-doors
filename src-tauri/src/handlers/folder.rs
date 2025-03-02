use std::{path::PathBuf, sync::Mutex};
use git2::Repository;
use tauri::{command, State};

use crate::core::{error::OpenDoorsError, folder::Folder, tree::TreeItem};

#[command] 
pub fn create_folder(state: State<'_, Mutex<Option<Repository>>>, name: String, parent: TreeItem) -> Result<TreeItem, OpenDoorsError> {
	let repo = state.lock().unwrap();
	Ok(Folder::create_folder(&repo, &name, &parent)?)
}

#[command]
pub fn read_folder(folder: TreeItem) -> Result<TreeItem, OpenDoorsError> {
	Ok(Folder::read(&folder)?)
}

#[command] 
pub fn update_folder(state: State<'_, Mutex<Option<Repository>>>, origin: PathBuf, destination: PathBuf) -> Result<TreeItem, OpenDoorsError> {
	let repo = state.lock().unwrap();
	Folder::update(&repo, &origin, &destination)?;
	Ok(TreeItem::from_path(&destination)?)
}

#[command] 
pub fn delete_folder(state: State<'_, Mutex<Option<Repository>>>, path: PathBuf) -> Result<(), OpenDoorsError> {
	let repo = state.lock().unwrap();
	Ok(Folder::delete(&repo, &path)?)
}
