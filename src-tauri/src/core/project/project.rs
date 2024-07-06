use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::core::{ProjectError, TreeItem};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub name: String,
    pub prefix: String,
    pub separator: String,
    pub location: PathBuf,
}

pub struct Project {
    manifest: ProjectManifest,
    tree: TreeItem,
}

impl Project {
    pub fn create(path: &PathBuf) -> Result<Project, ProjectError> {
        todo!()
    }
    
    pub fn read(path: &PathBuf) -> Result<Project, ProjectError> {
        todo!()
    }
    
    pub fn update(&self) -> Result<ProjectManifest, ProjectError> {
        todo!()
    }
    
    pub fn delete(&self) -> Result<(), ProjectError> {
        todo!()
    }
}
