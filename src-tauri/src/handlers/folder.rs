use std::path::PathBuf;

use tauri::command;

use crate::core::{error::OpenDoorsError, middleware as mid, tree::TreeItem};

#[command] 
pub fn create_folder(name: String, parent: TreeItem) -> Result<TreeItem, OpenDoorsError> {
    let path = mid::create_folder(&parent.path, &name)?;
    Ok(TreeItem::from_path(&path)?)
}

#[command]
pub fn read_folder(folder: TreeItem) -> Result<TreeItem, OpenDoorsError> {
  Ok(TreeItem::from_path(&folder.path)?)
}

#[command] 
pub fn update_folder(origin: PathBuf, destination: PathBuf) -> Result<TreeItem, OpenDoorsError> {
  mid::update_folder(&origin, &destination)?;
  Ok(TreeItem::from_path(&destination)?)
}

#[command] 
pub fn delete_folder(path: PathBuf) -> Result<(), OpenDoorsError> {
  Ok(mid::delete_folder(&path)?)
}
