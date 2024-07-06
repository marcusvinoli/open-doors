use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
}
