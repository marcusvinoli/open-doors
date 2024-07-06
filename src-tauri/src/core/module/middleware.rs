use std::io::{Read, Write};
use std::fs::{self, File};
use std::path::PathBuf;

use crate::core::{ModuleError, Repository};

use super::{module::{Module, ModuleManifest}, object::Object};

use super::definitions::{OD_OBJS_FOLDER_NAME, OD_DRAFT_FOLDER_NAME, OD_ASSETS_FOLDER_NAME};

pub fn create_objs_folder(module: &Module) -> Result<(), ModuleError> {
    let mut loc: PathBuf = module.path.clone();
    loc.push(OD_OBJS_FOLDER_NAME);
    fs::create_dir_all(loc)?;
    Ok(())
}

pub fn create_drafts_folder(module: &Module) -> Result<(), ModuleError> {
    let mut loc: PathBuf = module.path.clone();
    loc.push(OD_OBJS_FOLDER_NAME);
    loc.push(OD_DRAFT_FOLDER_NAME);
    fs::create_dir_all(loc)?;
    Ok(())
}

pub fn create_assets_folder(module: &Module) -> Result<(), ModuleError> {
    let mut loc: PathBuf = module.path.clone();
    loc.push(OD_OBJS_FOLDER_NAME);
    loc.push(OD_ASSETS_FOLDER_NAME);
    fs::create_dir_all(loc)?;
    Ok(())
}

pub fn create_module_folder(rep: &Repository, man: ModuleManifest) -> Result<Module, ModuleError> {
    let mut path: PathBuf = rep.manifest.path.clone();
    path.push(man.title.clone());
    fs::create_dir(&path)?;
    let module: Module = Module { 
        path, 
        baseline: Vec::new(), 
        manifest: man, 
        objects: Vec::new(), 
        drafts: Vec::new() 
    };
    Ok(module)
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

pub fn update_objs_folder(module: &mut Module) -> Result<(), ModuleError> {
    todo!()
}

pub fn update_draft_folder(module: &mut Module) -> Result<(), ModuleError> {
    todo!()
}