use std::path::PathBuf;
use tauri::command;

use crate::core::{error::OpenDoorsError, tree::TreeItem};

#[command]
pub fn read_tree(path: PathBuf) -> Result<TreeItem, OpenDoorsError> {
    if let Ok(x) = TreeItem::from_path(&path) {
        return Ok(x);
    }
    Err(OpenDoorsError::GenericError("Tree Error".into()))
}
