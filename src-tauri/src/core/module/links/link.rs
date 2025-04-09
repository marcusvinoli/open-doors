use serde::{Serialize, Deserialize};

use crate::core::utils::path_utils;

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Link {
    #[serde(with = "path_utils")]
    pub object: usize,
    pub path: String,
}
