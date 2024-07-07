use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use super::middleware as mid;

use crate::core::{RepositoryError, TreeItem, TreeItemType};
use crate::git::{self, GitError};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RepositoryManifest {
    pub name: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub remote: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Repository{
    pub manifest: RepositoryManifest,
    pub structure: TreeItem,
    pub path: PathBuf,
}

impl Repository {
    pub fn create(path: PathBuf, name: String, remote: Option<String>) -> Result<Repository, RepositoryError> {         
        if git::is_git_repository(path.display().to_string())? {
            return Err(RepositoryError::GitError(GitError::RepositoryNotEmpty));
        }

        let man: RepositoryManifest = RepositoryManifest {
            name: name.clone(),
            remote,
        };
        
        let tree: TreeItem = TreeItem {
            item_type: TreeItemType::Repository,
            path: PathBuf::from("./"),
            children: Vec::new(),
            name,
        };
        
        mid::create_manifest_file(&path, &man)?;
        mid::create_structure_file(&path, &tree)?;
        
        git::init(&path.display().to_string())?;
        git::add_all(&path.display().to_string())?;
        git::commit(&path.display().to_string(), "OpenDOORs repository initiated.")?;
        
        Ok(Repository { 
            path: path,
            manifest: man, 
            structure: tree
        })
    }
    
    pub fn read(path: PathBuf) -> Result<Repository, RepositoryError> {
        if !(git::is_git_repository(path.display().to_string())?) {
            return Err(RepositoryError::GitError(GitError::RepositoryNotEmpty));
        }
        
        let man: RepositoryManifest = mid::read_manifest_file(&path)?;
        let tree: TreeItem = mid::read_structure_file(&path)?;
        
        Ok(Repository { 
            path,
            manifest: man,
            structure: tree,
        })
    }
    
    pub fn update_manifest(path: PathBuf, man: RepositoryManifest) -> Result<(), RepositoryError> {
        if !(git::is_git_repository(&path.display().to_string())?) {
            return Err(RepositoryError::GitError(GitError::RepositoryNotEmpty));
        }

        mid::update_manifest_file(&path, &man)?;

        return Ok(());
    }
    
    pub fn update_structure(path: PathBuf, tree: TreeItem) -> Result<(), RepositoryError> {
        if !(git::is_git_repository(&path.display().to_string())?) {
            return Err(RepositoryError::GitError(GitError::RepositoryNotEmpty));
        }

        mid::update_structure_file(&path, &tree)?;

        return Ok(());
    }

    pub fn delete(path: PathBuf) -> Result<(), RepositoryError> {
        todo!("Delete operation not Implemented.");
        // mid::delete_manifest_file(&path)?;
        // mid::delete_structure_file(&path)?;
        // Ok(())
    }
}
