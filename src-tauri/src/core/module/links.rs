use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::core::utils::path_utils;

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Link {
    #[serde(with = "path_utils")]
    pub path: PathBuf,
    pub object: usize,
    pub module: String,
}
