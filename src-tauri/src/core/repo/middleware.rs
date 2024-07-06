use std::fs::{self, File};
use std::path::PathBuf;
use std::io::{Read, Write};

use crate::core::{RepositoryError, TreeItem};
use super::repository::RespositoryManifest;
use super::definitions::{OD_MANIFEST_FILE_NAME, OD_STRUCTURE_FILE_NAME};

pub fn create_manifest(man: &RespositoryManifest) -> Result<(), RepositoryError> {
    let manifest_content: String = serde_yaml::to_string(man).unwrap();
    let mut loc: PathBuf = man.path.clone();
    loc.push(OD_MANIFEST_FILE_NAME);
    File::create(loc)?.write_all(&mut manifest_content.into_bytes())?;
    Ok(())
}

pub fn read_manifest(repo_path: &PathBuf) -> Result<RespositoryManifest, RepositoryError> {
    let mut file_content: String = String::new();
    let mut loc: PathBuf = repo_path.clone();
    loc.push(OD_MANIFEST_FILE_NAME);
    File::open(loc)?.read_to_string(&mut file_content)?;
    let man: RespositoryManifest = serde_yaml::from_str(&file_content)?;
    Ok(man)
}

pub fn update_manifest(man: &RespositoryManifest) -> Result<(), RepositoryError> {
    create_manifest(man)
}

pub fn delete_manifest(man: RespositoryManifest) -> Result<(), RepositoryError> {
    fs::remove_dir_all(&man.path)?;
    Ok(())
}

pub fn create_structure_file(loc: &PathBuf, tree: &TreeItem) -> Result<(), RepositoryError> {
    let tree_content: String = serde_yaml::to_string(tree).unwrap();
    let mut loc: PathBuf = loc.clone();
    loc.push(OD_STRUCTURE_FILE_NAME);
    File::create(loc)?.write_all(&mut tree_content.into_bytes())?;
    Ok(())
}

pub fn read_structure_file(repo_loc: &PathBuf) -> Result<TreeItem, RepositoryError> {
    let mut file_content: String = String::new();
    let mut loc: PathBuf = repo_loc.clone();
    loc.push(OD_STRUCTURE_FILE_NAME);
    File::open(loc)?.read_to_string(&mut file_content)?;
    let tree: TreeItem = serde_yaml::from_str(&file_content)?;
    Ok(tree)
}

pub fn update_structure_file(loc: &PathBuf, tree: &TreeItem) -> Result<(), RepositoryError> {
    create_structure_file(loc, tree)?;
    Ok(())
}

pub fn delete_structure_file(loc: &PathBuf) -> Result<(), RepositoryError> {
    let mut loc = loc.clone();
    loc.push(OD_STRUCTURE_FILE_NAME);
    fs::remove_file(loc)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn od_manifest_creation() {
        let man: RespositoryManifest = RespositoryManifest{name: "Test Repo 1".into(), path: "./test".into(), remote: None};
        assert!(create_manifest(&man).is_ok());
    }
    
    #[test]
    fn od_manifest_reading() {
        let man: RespositoryManifest = RespositoryManifest{name: "Test Repo 1".into(), path: "./test/test1".into(), remote: Some(String::from("test"))};
        assert!(create_manifest(&man).is_ok());
        
        let res = read_manifest(&PathBuf::from("./test/test1"));
        assert!(res.is_ok());
        
        if let Ok(res) = res {
            assert_eq!(res.name, String::from("Test Repo 1"));
            assert_eq!(res.path, PathBuf::from("./test/test1"));
            assert_eq!(res.remote, Some(String::from("test")));
        }
    }

    #[test]
    fn od_manifest_updating() {
        let mut man: RespositoryManifest = RespositoryManifest{name: "Test Repo 2".into(), path: "./test/test2".into(), remote: Some(String::from("test"))};
        assert!(create_manifest(&man).is_ok());
        
        let res = read_manifest(&PathBuf::from("./test/test2"));
        assert!(res.is_ok());

        if let Ok(res) = res {
            assert_eq!(res.name, String::from("Test Repo 2"));
            assert_eq!(res.path, PathBuf::from("./test/test2"));
            assert_eq!(res.remote, Some(String::from("test")));
            man = res;
        }
        
        man.name = String::from("Hello World");
        man.remote = None;

        assert!(update_manifest(&man).is_ok());

        let res = read_manifest(&PathBuf::from("./test/test2"));
        assert!(res.is_ok());

        if let Ok(res) = res {
            assert_eq!(res.name, String::from("Hello World"));
            assert_eq!(res.path, PathBuf::from("./test/test2"));
            assert_eq!(res.remote, None);
        }

    }
}
