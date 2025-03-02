use std::path::PathBuf;

use git2::Repository;

use crate::core::{error::error::FolderError, git, middleware as mid, tree::TreeItem};
use super::definitions as defs;
pub struct Folder{}

impl Folder {
	pub fn create_folder(repo: &Option<Repository>, name: &str, parent: &TreeItem) -> Result<TreeItem, FolderError> {
		let repo = repo.as_ref().ok_or(FolderError::NoRepositoryInitialized)?;
		let path = mid::create_folder(&parent.path, &name)?;
		let dummy_file = mid::create_file(&path, defs::OD_FOLDER_DUMMY_FILENAME)?;
		git::add_file(&repo, &dummy_file.to_string_lossy())?;
		git::git_commit(&repo, &format!("Created folder `{}`.", name))?;
		Ok(TreeItem::from_path(&path)?)
	}
	pub fn read(folder: &TreeItem) -> Result<TreeItem, FolderError> {
		Ok(TreeItem::from_path(&folder.path)?)
	}

	pub fn update(repo: &Option<Repository>, origin: &PathBuf, destination: &PathBuf) -> Result<TreeItem, FolderError> {
		let repo = repo.as_ref().ok_or(FolderError::NoRepositoryInitialized)?;
		let mut repo_path = PathBuf::from(repo.path());
		repo_path.pop();
		let origin_rel = origin.strip_prefix(&repo_path).unwrap_or(origin).to_string_lossy();
		let destination_rel = destination.strip_prefix(&repo_path).unwrap_or(destination).to_string_lossy();
		mid::update_folder(&origin, &destination)?;
		git::add_folder(&repo, &origin.to_string_lossy())?;
		git::add_folder(&repo, &destination.to_string_lossy())?;
		git::git_commit(&repo, &format!("Updated folder from `{}` to `{}`.", origin_rel, destination_rel))?;
		Ok(TreeItem::from_path(&destination)?)
	}

	pub fn delete(repo: &Option<Repository>, path: &PathBuf) -> Result<(), FolderError> {
		let repo = repo.as_ref().ok_or(FolderError::NoRepositoryInitialized)?;
		let mut repo_path = PathBuf::from(repo.path());
		repo_path.pop();
		let path_rel = &path.strip_prefix(&repo_path).unwrap_or(&path).to_string_lossy();
		mid::delete_folder(&path)?;
		git::add_folder(&repo, &path.to_string_lossy())?;
		git::git_commit(&repo, &format!("Deleted folder `{}`.", path_rel))?;
		Ok(())
	}
}
