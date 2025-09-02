use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum AttributeKind {
    #[default]
    General, // Formatted text.
    String, // Text not formatted.
    Integer,
    Real,
    Date,
    Time,
    DateTime,
    Boolean,
    SingleOption(Vec<String>), // Single Option.
    MultipleOptions(Vec<String>), // Zero, One or More Itens from a list.
    User,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Attribute {
    pub is_mandatory: bool,
    pub kind: AttributeKind,
    pub name: String,
    pub description: String,
    pub key: String,
}
