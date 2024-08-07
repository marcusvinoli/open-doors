use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Link {
    pub path: PathBuf,
    pub object: usize,
    pub module: String,
}
