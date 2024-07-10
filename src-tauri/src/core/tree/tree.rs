use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
