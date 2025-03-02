use std::path::PathBuf;

use git2::Repository;
use serde::{Serialize, Deserialize};

use crate::core::{error::ProjectError, git, middleware as mid, tree::TreeItem};
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
	pub fn create(repo: &Option<Repository>, path: &str, man: &ProjectManifest) -> Result<Project, ProjectError> {
		let repo: &Repository = repo.as_ref().ok_or(ProjectError::NoRepositoryInitialized)?;
		let project_path: PathBuf = mid::create_folder(&path.into(), &man.prefix)?;
		let manifest_file = mid::create_yml_file(&project_path, defs::OD_PROJECT_MANIFEST_FILE_NAME, &man)?;
		git::add_file(&repo, &manifest_file.to_string_lossy())?;
		git::git_commit(&repo, &format!("Created project `{}`.", &man.name))?;
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
	
	pub fn update_manifest(repo: &Option<Repository>, path: &PathBuf, man: &ProjectManifest) -> Result<ProjectManifest, ProjectError> {
		Project::check_for_project_folder(&path)?;
		let repo: &Repository = repo.as_ref().ok_or(ProjectError::NoRepositoryInitialized)?;
		let file_updated = mid::update_yml_file(&path, defs::OD_PROJECT_MANIFEST_FILE_NAME,&man)?;
		git::add_file(&repo, &file_updated.to_string_lossy())?;
		git::git_commit(&repo, &format!("Updated project `{}` manifest.", &man.name))?;
		Ok(mid::read_yml_file::<ProjectManifest, _>(&path, defs::OD_PROJECT_MANIFEST_FILE_NAME)?)
	}
		
	pub fn delete(repo: &Option<Repository>, path: &PathBuf) -> Result<(), ProjectError> {
		let project: Project = Project::read(&path)?;
		let repo: &Repository = repo.as_ref().ok_or(ProjectError::NoRepositoryInitialized)?;
		mid::delete_folder(&path)?;
		git::add_folder(&repo, &path.to_string_lossy())?;
		git::git_commit(&repo, &format!("Deleted project `{}`.", &project.manifest.name))?;
		Ok(())
	}

	fn check_for_project_folder(path: &PathBuf) -> Result<(), ProjectError> {
		if !mid::file_exists(&path.join(defs::OD_PROJECT_MANIFEST_FILE_NAME))? {
			return Err(ProjectError::InvalidProjectDirectory)
		}
		Ok(())
	}
}
