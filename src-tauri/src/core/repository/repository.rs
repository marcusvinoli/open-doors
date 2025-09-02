use std::path::PathBuf;
use git2::Repository as GitRepository;
use serde::{Serialize, Deserialize};
use crate::core::git;

use crate::core::{middleware as mid, error::RepositoryError, tree::{TreeItem, TreeItemType}};

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
	pub fn create(repo: &mut Option<GitRepository>, path: &str, name: &str, remote: &Option<String>) -> Result<Repository, RepositoryError> {         
		let repo_path: PathBuf = mid::create_folder(&path.into(), name)?;
		let git_repository: GitRepository = git::init(&repo_path.to_string_lossy())?;
		let manifest: RepositoryManifest = RepositoryManifest {name: name.into()};

		mid::create_yml_file(&repo_path, defs::MANIFEST_FILE_NAME , &manifest)?;
		git::create_ignore_rules(&git_repository, &defs::IGNORE_RULES)?;
		git::add_file(&git_repository, ".gitignore")?;
		git::add_file(&git_repository, defs::MANIFEST_FILE_NAME)?;

		git::git_commit(&git_repository, "Repository Scaffolding.")?;

		if let Some(url) = remote {
			git::add_remote(&git_repository, defs::DEFAULT_BRANCH_NAME, url)?;
		}

		*repo = Some(git_repository);
		let path: String = repo_path.to_str().unwrap_or_default().into();
		let tree: TreeItem = TreeItem {
			path,
			name: name.into(),
			item_type: TreeItemType::Repository,
			children: Vec::new(),
		};

		Ok(Repository {
			manifest, 
			tree
		})
	}
	
	pub fn read(path: &PathBuf) -> Result<Repository, RepositoryError> {
		let man: RepositoryManifest = mid::read_yml_file(&path, defs::MANIFEST_FILE_NAME)?;
		let mut tree: TreeItem = TreeItem::from_path(&path)?;
		tree.name = man.name.clone();
		Ok(Repository { 
			manifest: man,
			tree,
		})
	}
	
	pub fn update(path: &PathBuf, man: RepositoryManifest) -> Result<(), RepositoryError> {
		mid::update_yml_file(&path, defs::MANIFEST_FILE_NAME, &man)?;
		return Ok(());
	}
	
	pub fn delete(path: &PathBuf) -> Result<(), RepositoryError> {
		Ok(mid::delete_folder(&path)?)
	}
}
