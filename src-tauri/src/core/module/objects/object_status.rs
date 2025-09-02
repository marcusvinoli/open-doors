use serde::{Serialize, Deserialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum ObjectStatus {
    #[default]
    Draft,
    Baselined,
    Updated,
    Deleted,
}
