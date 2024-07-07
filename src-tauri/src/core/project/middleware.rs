use std::io::{Read, Write};
use std::fs::{self, File};
use std::path::PathBuf;

use crate::core::{ProjectError, TreeItem};
use super::project::ProjectManifest;
use super::definitions::{OD_PROJECT_MANIFEST_FILE_NAME, OD_PROJ_STRUCTURE_FILE_NAME};

pub fn check_for_project_folder(path: &PathBuf) -> Result<(), ProjectError> {
    if path.join(OD_PROJECT_MANIFEST_FILE_NAME).exists() &&
        path.join(OD_PROJ_STRUCTURE_FILE_NAME).exists() {
        return Err(ProjectError::InvalidProjectDirectory)
    }
    Ok(())
}

pub fn create_project_folder(path: &PathBuf, man: &ProjectManifest) -> Result<PathBuf, ProjectError> {
    fs::create_dir(path.join(&man.prefix))?;
    Ok(path.clone())
}

pub fn delete_project_folder(path: &PathBuf) -> Result<(), ProjectError> {
    Ok(fs::remove_dir_all(path)?)
}

pub fn create_project_manifest(path: &PathBuf, man: &ProjectManifest) -> Result<(), ProjectError> {
    let manifest_content: String = serde_yaml::to_string(man)?;
    File::create(path.join(OD_PROJECT_MANIFEST_FILE_NAME))?.write_all(&mut manifest_content.into_bytes())?;
    Ok(())
}

pub fn read_project_manifest(path: &PathBuf) -> Result<ProjectManifest, ProjectError> {
    let mut file_content: String = String::new();
    File::open(path.join(OD_PROJECT_MANIFEST_FILE_NAME))?.read_to_string(&mut file_content)?;
    let man: ProjectManifest = serde_yaml::from_str(&file_content)?;
    Ok(man)
}

pub fn update_project_manifest(path: &PathBuf, man: &ProjectManifest) -> Result<(), ProjectError> {
    create_project_manifest(path, man)
}

pub fn delete_project_manifest(path: &PathBuf) -> Result<(), ProjectError> {
    fs::remove_file(path.join(OD_PROJECT_MANIFEST_FILE_NAME))?;
    Ok(())
}

pub fn create_project_structure(path: &PathBuf, tree: &TreeItem) -> Result<(), ProjectError> {
    let tree_content: String = serde_yaml::to_string(tree).unwrap();
    File::create(path.join(OD_PROJ_STRUCTURE_FILE_NAME))?.write_all(&mut tree_content.into_bytes())?;
    Ok(())
}

pub fn read_project_structure(path: &PathBuf) -> Result<TreeItem, ProjectError> {
    let mut file_content: String = String::new();
    File::open(path.join(OD_PROJ_STRUCTURE_FILE_NAME))?.read_to_string(&mut file_content)?;
    let tree: TreeItem = serde_yaml::from_str(&file_content)?;
    Ok(tree)
}

pub fn update_project_structure(path: &PathBuf, tree: &TreeItem) -> Result<(), ProjectError> {
    create_project_structure(path, tree)
}

pub fn delete_project_structure(path: &PathBuf) -> Result<(), ProjectError> {
    Ok(fs::remove_file(path.join(OD_PROJ_STRUCTURE_FILE_NAME))?)
}