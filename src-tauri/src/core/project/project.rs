use std::{ffi::OsStr, path::PathBuf};

use serde::{Serialize, Deserialize};

use crate::core::{ProjectError, TreeItem, TreeItemType};

use super::middleware as mid;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub name: String,
    pub prefix: String,
    pub separator: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Project {
    pub manifest: ProjectManifest,
    pub tree: TreeItem,
    pub path: PathBuf,
}

impl Project {
    pub fn create(path: &PathBuf, man: &ProjectManifest) -> Result<Project, ProjectError> {
        let prj_path = mid::create_project_folder(path, man)?;
        mid::create_project_manifest(&prj_path, man)?;
        let tree: TreeItem = TreeItem { 
            item_type: TreeItemType::Project, 
            name: man.name.clone(), 
            path: PathBuf::from(prj_path.file_name().unwrap_or(OsStr::new("./"))), 
            children: Vec::new() 
        };
        mid::create_project_structure(&prj_path,  &tree)?;
        Ok(Project { 
            manifest: man.clone(), 
            tree, 
            path: path.join(&man.prefix) 
        })
    }
    
    pub fn read(path: &PathBuf) -> Result<Project, ProjectError> {
        mid::check_for_project_folder(path)?;
        let manifest: ProjectManifest = mid::read_project_manifest(path)?;
        let tree: TreeItem = mid::read_project_structure(path)?;
        Ok(Project {
            path: path.clone(),
            manifest,
            tree,
        })
    }
    
    pub fn update_manifest(path: &PathBuf, man: ProjectManifest) -> Result<ProjectManifest, ProjectError> {
        mid::check_for_project_folder(&path)?;
        mid::update_project_manifest(&path, &man)?;
        mid::read_project_manifest(&path)
    }
    
    pub fn update_structure(path: &PathBuf, tree: TreeItem) -> Result<TreeItem, ProjectError> {
        mid::check_for_project_folder(&path)?;
        mid::update_project_structure(&path, &tree)?;
        mid::read_project_structure(&path)
    }
    
    pub fn delete(path: &PathBuf) -> Result<(), ProjectError> {
        mid::check_for_project_folder(&path)?;
        mid::delete_project_folder(&path)
    }
}
