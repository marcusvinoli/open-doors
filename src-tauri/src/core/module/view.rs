use serde::{Serialize, Deserialize};

use super::Attribute;
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct View {
    pub fields: Vec<Attribute>
}
