use std::path::PathBuf;
use tauri::command; 

use crate::core::{error::OpenDoorsError, module::{object::Object, template::Template, Module, ModuleManifest}, tree::TreeItem};

#[command] 
pub fn create_module(man: ModuleManifest, parent: TreeItem) -> Result<Module, OpenDoorsError> {
    Ok(Module::create(&parent.path, &man)?)
}

#[command]
pub fn read_module(path: PathBuf) -> Result<Module, OpenDoorsError> {
    Ok(Module::read(&path)?)
}

#[command]
pub fn create_object(path: PathBuf, object: Object) -> Result<Object, OpenDoorsError> {
    let mut module = Module::read(&path)?;
    let mut obj = object.clone();
    Ok(module.create_object(&mut obj)?)
}

#[command]
pub fn create_draft_object(path: PathBuf, object: Object) -> Result<Object, OpenDoorsError> {
    let mut module = Module::read(&path)?;
    let mut obj = object.clone();
    Ok(module.create_draft_object(&mut obj)?)
}

#[command]
pub fn read_object(path: PathBuf, id: usize) -> Result<Object, OpenDoorsError> {
    let module = Module::read(&path)?;
    Ok(module.read_object(id)?)
}

#[command]
pub fn read_draft_object(path: PathBuf, id: usize) -> Result<Object, OpenDoorsError> {
    let module = Module::read(&path)?;
    Ok(module.read_draft_object(id)?)
}

#[command]
pub fn read_objects(path: PathBuf) -> Result<Vec<Object>, OpenDoorsError> {
    let mut module = Module::read(&path)?;
    Ok(module.read_objects()?)
}

#[command]
pub fn read_draft_objects(path: PathBuf) -> Result<Vec<Object>, OpenDoorsError> {
    let mut module = Module::read(&path)?;
    Ok(module.read_draft_objects()?)
}

#[command] 
pub fn update_object(path: PathBuf, object: Object) -> Result<Object, OpenDoorsError> {
    let mut module = Module::read(&path)?;
    let mut obj = object.clone();
    Ok(module.update_object(&mut obj)?) 
}

#[command] 
pub fn update_draft_object(path: PathBuf, object: Object) -> Result<Object, OpenDoorsError> {
    let mut module = Module::read(&path)?;
    let mut obj = object.clone();
    Ok(module.update_draft_object(&mut obj)?)
}

#[command] 
pub fn delete_object(path: PathBuf, id: usize) -> Result<Object, OpenDoorsError> {
    let mut module = Module::read(&path)?;
    Ok(module.delete_object(id)?)
}

#[command]
pub fn create_template(path: PathBuf, template: Template) -> Result<Template, OpenDoorsError> {
    let module = Module::read(&path)?;
    Ok(module.create_template(template)?)
}

#[command]
pub fn read_template(path: PathBuf) -> Result<Template, OpenDoorsError> {
    let module = Module::read(&path)?;
    Ok(module.read_template()?)
}

#[command]
pub fn update_template(path: PathBuf, template: Template) -> Result<Template, OpenDoorsError> {
    let module = Module::read(&path)?;
    Ok(module.update_template(template)?)
}
