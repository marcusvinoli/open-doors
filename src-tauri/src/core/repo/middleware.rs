use std::fs::{self, File};
use std::path::PathBuf;
use std::io::{Read, Write};

use crate::core::{RepositoryManifest, RepositoryError, TreeItem};
use super::definitions::{OD_MANIFEST_FILE_NAME, OD_STRUCTURE_FILE_NAME};

pub fn create_manifest_file(path: &PathBuf, man: &RepositoryManifest) -> Result<(), RepositoryError> {
    let manifest_content: String = serde_yaml::to_string(man).unwrap();
    let mut loc: PathBuf = path.clone();
    loc.push(OD_MANIFEST_FILE_NAME);
    File::create(loc)?.write_all(&mut manifest_content.into_bytes())?;
    Ok(())
}

pub fn read_manifest_file(path: &PathBuf) -> Result<RepositoryManifest, RepositoryError> {
    let mut file_content: String = String::new();
    let mut loc: PathBuf = path.clone();
    loc.push(OD_MANIFEST_FILE_NAME);
    File::open(loc)?.read_to_string(&mut file_content)?;
    let man: RepositoryManifest = serde_yaml::from_str(&file_content)?;
    Ok(man)
}

pub fn update_manifest_file(path: &PathBuf, man: &RepositoryManifest) -> Result<(), RepositoryError> {
    create_manifest_file(path, man)
}

pub fn delete_manifest_file(path: &PathBuf) -> Result<(), RepositoryError> {
    let mut loc: PathBuf = path.clone();
    loc.push(OD_MANIFEST_FILE_NAME);
    fs::remove_dir_all(&loc)?;
    Ok(())
}

pub fn create_structure_file(path: &PathBuf, tree: &TreeItem) -> Result<(), RepositoryError> {
    let tree_content: String = serde_yaml::to_string(tree).unwrap();
    let mut loc: PathBuf = path.clone();
    loc.push(OD_STRUCTURE_FILE_NAME);
    File::create(loc)?.write_all(&mut tree_content.into_bytes())?;
    Ok(())
}

pub fn read_structure_file(path: &PathBuf) -> Result<TreeItem, RepositoryError> {
    let mut file_content: String = String::new();
    let mut loc: PathBuf = path.clone();
    loc.push(OD_STRUCTURE_FILE_NAME);
    File::open(loc)?.read_to_string(&mut file_content)?;
    let tree: TreeItem = serde_yaml::from_str(&file_content)?;
    Ok(tree)
}

pub fn update_structure_file(loc: &PathBuf, tree: &TreeItem) -> Result<(), RepositoryError> {
    create_structure_file(loc, tree)?;
    Ok(())
}

pub fn delete_structure_file(path: &PathBuf) -> Result<(), RepositoryError> {
    let mut loc: PathBuf = path.clone();
    loc.push(OD_STRUCTURE_FILE_NAME);
    fs::remove_file(loc)?;
    Ok(())
}

pub fn delete_repositoru(path: &PathBuf) -> Result<(), RepositoryError> {
    Ok(fs::remove_dir_all(path)?)
}
