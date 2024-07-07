use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::core::ProjectError;

use super::{baseline::Baseline, object::Object};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModuleManifest {
    pub title: String,
    pub prefix: String,
    pub separator: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Module{
    pub path: PathBuf,
    pub baseline: Vec<Baseline>,
    pub manifest: ModuleManifest,
    pub objects: Vec<Object>,
    pub drafts: Vec<Object>,
}

impl Module {
    pub fn create(path: &PathBuf) -> Result<Module, ProjectError> {
        todo!()
    }
    
    pub fn read(path: &PathBuf) -> Result<Module, ProjectError> {
        todo!()
    }
    
    pub fn update(&self) -> Result<Module, ProjectError> {
        todo!()
    }
    
    pub fn delete(self) -> Result<(), ProjectError> {
        todo!()
    }
}
