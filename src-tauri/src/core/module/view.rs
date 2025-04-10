use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ViewItem {
    pub attribute: String,
    pub key: String,
    pub show: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct View {
    pub name: String,
    pub fields: Vec<ViewItem>,
}
