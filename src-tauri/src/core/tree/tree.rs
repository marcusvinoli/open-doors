use std::fs;
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::core::middleware::error::MiddlewareError;
use crate::core::project;
use crate::core::module;
use crate::core::repository;

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum TreeItemType {
    Repository,
    Project,
    Folder,
    Module,
    #[default]
    Other,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct TreeItem {
    pub item_type: TreeItemType,
    pub name: String,
    pub path: PathBuf,
    pub children: Vec<TreeItem>,
}

impl TreeItem {
    pub fn from_path(path: &PathBuf) -> Result<TreeItem, MiddlewareError> {
        fn determine_item_type(path: &PathBuf) -> TreeItemType {
            if path.join(repository::definitions::OD_REPO_MANIFEST_FILE_NAME).exists() {
                TreeItemType::Repository
            } else if path.join(module::definitions::OD_MODULE_MANIFEST_FILE_NAME).exists() {
                TreeItemType::Module
            } else if path.join(project::definitions::OD_PROJECT_MANIFEST_FILE_NAME).exists() {
                TreeItemType::Project
            } else {
                TreeItemType::Folder
            }
        }

        fn read_children(path: &PathBuf, item_type: &TreeItemType) -> Result<Vec<TreeItem>, MiddlewareError> {
            if matches!(item_type, TreeItemType::Module) {
                // If the item type is Module, return an empty Vec.
                return Ok(Vec::new());
            }
            
            let mut children = Vec::new();
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let entry_path = entry.path();
                if entry_path.is_dir() && !entry_path.ends_with(".git") {
                    children.push(TreeItem::from_path(&entry_path)?);
                }
            }
            Ok(children)
        }

        let item_type = determine_item_type(&path);
        let children = read_children(&path, &item_type)?;
        let name = path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
            .to_string();

        Ok(TreeItem {
            name,
            path: path.clone(),
            item_type,
            children,
        })
    }
}