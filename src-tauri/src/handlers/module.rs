use std::{path::PathBuf, sync::Mutex};
use git2::Repository as GitRepository;
use tauri::{command, State}; 

use crate::core::{error::OpenDoorsError, module::{Baseline, Object, Template, Module, ModuleManifest}, tree::TreeItem};

#[command] 
pub fn create_module(state: State<'_, Mutex<Option<GitRepository>>>, man: ModuleManifest, parent: TreeItem) -> Result<Module, OpenDoorsError> {
	let repo = state.lock().unwrap();
	Ok(Module::create(&repo, &parent.path.into(), &man)?)
}

#[command]
pub fn read_module(path: PathBuf) -> Result<Module, OpenDoorsError> {
	Ok(Module::read(&path)?)
}

#[command]
pub fn update_module(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf, man: ModuleManifest) -> Result<Module, OpenDoorsError> {
	let repo = state.lock().unwrap();
	Module::update(&repo, &path, &man)?;
	Ok(Module::read(&path)?)
}

#[tauri::command]
pub fn delete_module(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf) -> Result<(), String> {
	let repo = state.lock().unwrap();
  	Ok(())
}

#[command]
pub fn create_object(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf, object: Object) -> Result<Object, OpenDoorsError> {
	let mut module = Module::read(&path)?;
	let repo = state.lock().unwrap();
	Ok(module.create_object(&repo, &mut object.clone())?)
}

#[command]
pub fn create_draft_object(path: PathBuf, object: Object) -> Result<Object, OpenDoorsError> {
	let mut module = Module::read(&path)?;
	Ok(module.create_draft_object(&mut &mut object.clone())?)
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
	let module = Module::read(&path)?;
	Ok(module.read_objects()?)
}

#[command]
pub fn read_draft_objects(path: PathBuf) -> Result<Vec<Object>, OpenDoorsError> {
	let mut module = Module::read(&path)?;
	Ok(module.read_draft_objects()?)
}

#[command] 
pub fn update_object(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf, object: Object) -> Result<Object, OpenDoorsError> {
	let repo = state.lock().unwrap();
	let mut module = Module::read(&path)?;
	let mut obj = object.clone();
	Ok(module.update_object(&repo, &mut obj)?) 
}

#[command] 
pub fn update_draft_object(path: PathBuf, object: Object) -> Result<Object, OpenDoorsError> {
	let mut module = Module::read(&path)?;
	let mut obj = object.clone();
	Ok(module.update_draft_object(&mut obj)?)
}

#[command] 
pub fn delete_object(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf, id: usize) -> Result<Object, OpenDoorsError> {
	let repo = state.lock().unwrap();
	let mut module = Module::read(&path)?;
	Ok(module.delete_object(&repo, id)?)
}

#[command] 
pub fn restore_object(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf, id: usize) -> Result<Object, OpenDoorsError> {
	let repo = state.lock().unwrap();
	let mut module = Module::read(&path)?;
	Ok(module.restore_object(&repo, id)?)
}

#[command]
pub fn create_template(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf, template: Template) -> Result<Template, OpenDoorsError> {
	let repo = state.lock().unwrap();
	let module = Module::read(&path)?;
	Ok(module.create_template(&repo, template)?)
}

#[command]
pub fn read_template(path: PathBuf) -> Result<Template, OpenDoorsError> {
	let module = Module::read(&path)?;
	Ok(module.read_template()?)
}

#[command]
pub fn update_template(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf, template: Template) -> Result<Template, OpenDoorsError> {
	let repo = state.lock().unwrap();
	let module = Module::read(&path)?;
	Ok(module.update_template(&repo, template)?)
}

#[command]
pub fn create_baseline(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf, baseline: Baseline) -> Result<(), OpenDoorsError> {
	let repo = state.lock().unwrap();
	let mut module = Module::read(&path)?;
	module.create_baseline(&repo, &baseline.version.to_string(), &baseline.description)?;
	Ok(())
}

#[command]
pub fn read_object_from_baseline(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf, id: usize, version: String) -> Result<Object, OpenDoorsError> {
	let repo = state.lock().unwrap();
	let module = Module::read(&path.into())?;
	Ok(module.read_object_from_baseline(&repo, &id, &version)?)
}

#[command]
pub fn read_objects_from_baseline(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf, version: String) -> Result<Vec<Object>, OpenDoorsError> {
	let repo = state.lock().unwrap();
	let module = Module::read(&path.into())?;
	Ok(module.read_objects_from_baseline(&repo, &version)?)
}
