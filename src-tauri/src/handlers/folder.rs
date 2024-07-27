use tauri::command; 

use crate::core::{middleware::{self as mid, error::MiddlewareError}, tree::TreeItem};

#[command] 
pub fn create_folder(name: String, parent: TreeItem) -> Result<TreeItem, MiddlewareError> {
    let path = mid::create_folder(&parent.path, &name)?;
    Ok(TreeItem::from_path(&path)?)
}

#[command]
pub fn read_folder(folder: TreeItem) -> Result<TreeItem, MiddlewareError> {
  Ok(TreeItem::from_path(&folder.path)?)
}
