use std::path::PathBuf;
use tauri::command; 

use crate::core::{error::OpenDoorsError, module::{Module, ModuleManifest}, tree::TreeItem};

#[command] 
pub fn create_module(man: ModuleManifest, parent: TreeItem) -> Result<Module, OpenDoorsError> {
    Ok(Module::create(&parent.path, &man)?)
}

#[command]
pub fn read_module(path: PathBuf) -> Result<Module, OpenDoorsError> {
  Ok(Module::read(&path)?)
}
