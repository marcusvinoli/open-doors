use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::core::{middleware as mid, error::RepositoryError, tree::{TreeItem, TreeItemType}};
use crate::git::{self, GitError};
use super::definitions as defs;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct RepositoryManifest {
    pub name: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Repository{
    pub manifest: RepositoryManifest,
    pub tree: TreeItem,
}

impl Repository {
    pub fn create(path: &PathBuf, name: &String, remote: &Option<String>) -> Result<Repository, RepositoryError> {         
        if git::is_git_repository(path.display().to_string())? {
            return Err(RepositoryError::GitError(GitError::RepositoryNotEmpty));
        }
        
        let repo_path = mid::create_folder(&path, name)?;
    
        let man: RepositoryManifest = RepositoryManifest {
            name: name.clone(),
        };
        
        let tree: TreeItem = TreeItem {
            name: name.clone(),
            path: repo_path.clone(),
            item_type: TreeItemType::Repository,
            children: Vec::new(),
        };
    
        mid::create_yml_file(&repo_path, defs::OD_REPO_MANIFEST_FILE_NAME , &man)?;
    
        git::init(&repo_path.display().to_string())?;
        git::add_all(&repo_path.display().to_string())?;
        git::commit(&repo_path.display().to_string(), "OpenDOORs repository initiated.")?;
        
        Ok(Repository {
            manifest: man, 
            tree
        })
    }
    
    pub fn read(path: &PathBuf) -> Result<Repository, RepositoryError> {
        if !(git::is_git_repository(path.display().to_string())?) {
            return Err(RepositoryError::GitError(GitError::RepositoryNotEmpty));
        }
    
        let man: RepositoryManifest = mid::read_yml_file(&path, defs::OD_REPO_MANIFEST_FILE_NAME)?;
        let tree: TreeItem = TreeItem::from_path(&path)?;
    
        Ok(Repository { 
            manifest: man,
            tree,
        })
    }
    
    pub fn update(path: &PathBuf, man: RepositoryManifest) -> Result<(), RepositoryError> {
        if !(git::is_git_repository(&path.display().to_string())?) {
            return Err(RepositoryError::GitError(GitError::RepositoryNotEmpty));
        }
    
        mid::update_yml_file(&path, defs::OD_REPO_MANIFEST_FILE_NAME, &man)?;
    
        return Ok(());
    }
    
    pub fn delete(path: &PathBuf) -> Result<(), RepositoryError> {
        Ok(mid::delete_folder(&path)?)
    }
}
