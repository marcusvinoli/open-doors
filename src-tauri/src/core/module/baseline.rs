use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Baseline {
    #[serde(skip_serializing_if="Option::is_none")]
    commit: Option<String>,
    version: String,
    description: String,
}
