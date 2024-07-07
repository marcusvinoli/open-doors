use std::io::{Read, Write};
use std::fs::{self, File};
use std::path::PathBuf;

use crate::core::{ModuleError, ModuleManifest, Module};
use super::{definitions as def, object::Object};

pub fn check_for_manifest_project(path: &PathBuf) -> Result<(), ModuleError> {
    if path.join(def::OD_MODULE_FILE_NAME).exists() {
        return Err(ModuleError::InvalidModuleDirectory)
    }
    Ok(())
}

pub fn create_module_folder(path: &PathBuf, man: &ModuleManifest) -> Result<Module, ModuleError> {
    fs::create_dir(&path.join(&man.prefix))?;

    Ok(Module { 
        path: path.join(&man.prefix), 
        manifest: man.clone(), 
        baseline: Vec::new(), 
        objects: Vec::new(), 
        drafts: Vec::new() 
    })
}

pub fn create_objs_folder(module: &Module) -> Result<(), ModuleError> {
    Ok(fs::create_dir(module.path.join(def::OD_OBJS_FOLDER_NAME))?)
}

pub fn create_drafts_folder(module: &Module) -> Result<(), ModuleError> {
    Ok(fs::create_dir(module.path.join(def::OD_OBJS_FOLDER_NAME).join(def::OD_DRAFT_FOLDER_NAME))?)
}

pub fn create_assets_folder(module: &Module) -> Result<(), ModuleError> {
    Ok(fs::create_dir(module.path.join(def::OD_OBJS_FOLDER_NAME).join(def::OD_ASSETS_FOLDER_NAME))?)
}

pub fn create_module_manifest(path: &PathBuf, man: &ModuleManifest) -> Result<(), ModuleError> {
    let manifest_content: String = serde_yaml::to_string(man)?;
    File::create(path.join(def::OD_MODULE_FILE_NAME))?.write_all(&mut manifest_content.into_bytes())?;
    Ok(())
}

pub fn read_objs_folder(module: &mut Module) -> Result<Vec<Object>, ModuleError> {
    todo!()
}

pub fn read_drafts_folder(module: &mut Module) -> Result<Vec<Object>, ModuleError> {
    todo!()
}

pub fn read_module_folder(module: &Module) -> Result<(), ModuleError> {
    todo!()
}

pub fn read_module_manifest(path: &PathBuf) -> Result<ModuleManifest, ModuleError> {
    let mut file_content: String = String::new();
    File::open(path.join(def::OD_MODULE_FILE_NAME))?.read_to_string(&mut file_content)?;
    let man: ModuleManifest = serde_yaml::from_str(&file_content)?;
    Ok(man)
}

pub fn update_objs_folder(module: &mut Module) -> Result<(), ModuleError> {
    todo!()
}

pub fn update_draft_folder(module: &mut Module) -> Result<(), ModuleError> {
    todo!()
}

pub fn update_module_manifest(path: &PathBuf, man: &ModuleManifest) -> Result<ModuleManifest, ModuleError> {
    todo!()
}