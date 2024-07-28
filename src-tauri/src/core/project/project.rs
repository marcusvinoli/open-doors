use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::core::{error::ProjectError, middleware as mid, tree::TreeItem};
use super::definitions as defs;

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
}

impl Project {
    pub fn create(path: &PathBuf, man: &ProjectManifest) -> Result<Project, ProjectError> {
        let project_path = mid::create_folder(&path, &man.prefix)?;
        mid::create_yml_file(&project_path, defs::OD_PROJECT_MANIFEST_FILE_NAME, &man)?;
        let project = Project {
            manifest: man.clone(),
            tree: TreeItem::from_path(&project_path)?,
        };
        Ok(project)
    }
    
    pub fn read(path: &PathBuf) -> Result<Project, ProjectError> {
        Project::check_for_project_folder(&path)?;

        let manifest: ProjectManifest = mid::read_yml_file(path, defs::OD_PROJECT_MANIFEST_FILE_NAME)?;
        let tree: TreeItem = TreeItem::from_path(path)?;
        
        Ok(Project {
            manifest,
            tree,
        })
    }
    
    pub fn update_manifest(path: &PathBuf, man: &ProjectManifest) -> Result<ProjectManifest, ProjectError> {
        Project::check_for_project_folder(&path)?;
        mid::update_yml_folder(&path, defs::OD_PROJECT_MANIFEST_FILE_NAME,&man)?;
        Ok(mid::read_yml_file::<ProjectManifest, _>(&path, defs::OD_PROJECT_MANIFEST_FILE_NAME)?)
    }
        
    pub fn delete(path: &PathBuf) -> Result<(), ProjectError> {
        Project::check_for_project_folder(&path)?;
        Ok(mid::delete_folder(&path)?)
    }

    fn check_for_project_folder(path: &PathBuf) -> Result<(), ProjectError> {
        if !mid::file_exists(&path.join(defs::OD_PROJECT_MANIFEST_FILE_NAME))? {
            return Err(ProjectError::InvalidProjectDirectory)
        }
        Ok(())
    }
}
