use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,
}
