use std::{path::PathBuf, sync::Mutex};

use git2::Repository as GitRepository;
use tauri::{command, State};

use crate::core::{error::OpenDoorsError, tree::TreeItem};

#[command]
pub fn read_tree(state: State<'_, Mutex<Option<GitRepository>>>) -> Result<TreeItem, OpenDoorsError> {
	if let Some(ref repo) = *state.lock().unwrap() {
		println!("Got here!");
		return Ok(TreeItem::from_path(&PathBuf::from(repo.path()))?);
	}
	Err(OpenDoorsError::GenericError("Tree Error".into()))
}
