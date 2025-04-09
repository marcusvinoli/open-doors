use serde::{Serialize, Deserialize};

use crate::core::utils::path_utils;

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Link {
    pub object: usize,
    #[serde(with = "path_utils")]
    pub path: String,
}

impl Link {
    pub fn module(&self) -> String {
        let path = self.path.to_owned();
        path.clone().rsplit('/').next().unwrap_or(&self.path).into()
    }
}
