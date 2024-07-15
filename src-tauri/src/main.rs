#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod core;
pub mod git;
mod state;

use std::path::PathBuf;

use core::{OpenDoorsError, Project, ProjectManifest, Repository, RepositoryManifest, TreeItem, TreeItemType};

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
		clone_repo,
		read_repo,
		create_repo,
		create_project,
		update_structure_file,
		read_structure_file,
		])
		.run(tauri::generate_context!())
		.expect("Error while running OpenDOORs.");
}

#[tauri::command]
fn clone_repo(man: RepositoryManifest, path: PathBuf) -> Result<Repository, OpenDoorsError> {
	git::clone(man.remote.unwrap_or_default().clone(), path.clone().display().to_string().clone())?;
	read_repo(path)
}

#[tauri::command]
fn read_repo(path: PathBuf) -> Result<Repository, OpenDoorsError> {
	Ok(Repository::read(path)?)
}

#[tauri::command]
fn create_repo(man: RepositoryManifest, path: PathBuf) -> Result<Repository, OpenDoorsError> {
	Ok(Repository::create(path, man.name, man.remote)?)
}

#[tauri::command]
fn create_project(path: PathBuf, man: ProjectManifest) -> Result<Project, OpenDoorsError> {
	Ok(Project::create(&path, &man)?)
}

#[tauri::command]
fn read_structure_file(path: PathBuf, parent: TreeItem) -> Result<TreeItem, OpenDoorsError> {
	let parent_update: TreeItem;
	match parent.item_type {
		TreeItemType::Repository => {
			let repo = Repository::read(path)?;
			parent_update = repo.structure;
		},
		TreeItemType::Folder | TreeItemType::Project => {
			let prj = Project::read(&path.join(parent.path))?;
			parent_update = prj.tree;
		},
		_ => {
			return Err(OpenDoorsError::GenericError("Indexing error".into()))
		}
	}
	Ok(parent_update)
}

#[tauri::command]
fn update_structure_file(path: PathBuf, child: TreeItem, parent: TreeItem) -> Result<TreeItem, OpenDoorsError> {
	let mut parent_update: TreeItem = parent.clone();
	match parent.item_type {
		TreeItemType::Repository => {
			let mut repo = Repository::read(path.join(parent.path))?;
			repo.structure.children.push(child);
			parent_update = repo.structure.clone();
			Repository::update_structure(path, repo.structure)?;
		},
		TreeItemType::Folder => {
			let mut prj = Project::read(&path.join(parent.path))?;
			if prj.tree.children.len() == 0 {
				prj.tree.children.push(child);
				parent_update = prj.tree;
			} else {
				let mut add_child_to_parent = |tree_item: &mut TreeItem, parent_name: &str, child: TreeItem| -> bool {
					let mut found = false;
			
					let mut visit = |node: &mut TreeItem| {
						if node.name == parent_name {
							node.children.push(child.clone());
							parent_update = node.clone();
							found = true;
						}
					};
			
					fn visit_all(tree_item: &mut TreeItem, parent_name: &str, visit: &mut impl FnMut(&mut TreeItem)) {
						visit(tree_item);
						for child in &mut tree_item.children {
							visit_all(child, parent_name, visit);
						}
					}
			
					visit_all(tree_item, parent_name, &mut visit);
					found
				};
				add_child_to_parent(&mut prj.tree, &parent.name, child);
				Project::update_structure(&prj.path, prj.tree)?;
			}
		},
		TreeItemType::Project => {
			let mut prj = Project::read(&path.join(parent.path))?;
			prj.tree.children.push(child);
			parent_update = prj.tree.clone();
			Project::update_structure(&prj.path, prj.tree)?;
		}
		_ => return Err(OpenDoorsError::GenericError("Indexing error".into()))
	}
	Ok(parent_update)
}

/*
// REPOSITORY
fn update_repo() {}
fn delete_repo() {}

// PROJECTS
fn read_project() {}
fn create_project() {}
fn delete_project() {}
fn update_project() {}

// FOLDER
fn read_folder() {}
fn create_folder() {}
fn delete_folder() {}
fn update_folder() {}

// MODULE
fn read_module() {}
fn create_module() {}
fn update_module() {}
fn delete_module() {}

// OBJECTS
fn read_object() {}
fn create_object() {}
fn update_object() {}
fn delete_object() {}

*/
