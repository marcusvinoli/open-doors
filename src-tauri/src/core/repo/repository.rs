use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use super::middleware as mid;

use crate::core::repo::definitions::OD_MANIFEST_FILE_NAME;
use crate::core::{RepositoryError, TreeItem, TreeItemType};
use crate::git::{self, GitError};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RespositoryManifest {
    pub name: String,
    pub path: PathBuf,
    #[serde(skip_serializing_if="Option::is_none")]
    pub remote: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Respository{
    pub manifest: RespositoryManifest,
    pub structure: TreeItem,
}

impl Respository {
    pub fn create(path: &PathBuf, name: String, remote: Option<String>) -> Result<Respository, RepositoryError> {         
        if git::is_git_repository(path.display().to_string())? {
            return Err(RepositoryError::GitError(GitError::RepositoryNotEmpty));
        }

        let man: RespositoryManifest = RespositoryManifest {
            name: name.clone(),
            path: path.clone(),
            remote,
        };

        let tree: TreeItem = TreeItem {
            item_type: TreeItemType::Repository,
            name,
            path: path.clone(),
            children: Vec::new(),
        };

        mid::create_manifest(&man)?;
        mid::create_structure_file(&man.path, &tree)?;

        git::init(&path.display().to_string())?;
        git::add_all(&path.display().to_string())?;
        git::commit(&path.display().to_string(), "OpenDOORs repository initiated.")?;

        Ok(Respository { 
            manifest: man, 
            structure: tree
        })
    }
    
    pub fn read(path: &PathBuf) -> Result<Respository, RepositoryError> {
        if !(git::is_git_repository(path.display().to_string())?) {
            return Err(RepositoryError::GitError(GitError::RepositoryNotEmpty));
        }
        
        let man: RespositoryManifest = mid::read_manifest(path)?;
        let tree: TreeItem = mid::read_structure_file(&man.path)?;
        
        Ok(Respository { 
            manifest: man,
            structure: tree,
        })
    }
    
    pub fn update(&self) -> Result<(), RepositoryError> {
        if !(git::is_git_repository(&self.manifest.path.display().to_string())?) {
            return Err(RepositoryError::GitError(GitError::RepositoryNotEmpty));
        }

        mid::update_manifest(&self.manifest)?;

        return Ok(());
    }
    
    pub fn delete(self) -> Result<(), RepositoryError> {
        mid::delete_manifest(self.manifest)
    }
}
