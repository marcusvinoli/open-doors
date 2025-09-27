use std::sync::LazyLock;

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

pub static READ_ONLY_ATTRIBUTES: LazyLock<Vec<Attribute>> = LazyLock::new(|| vec![
    Attribute { 
        key: String::from("id"), 
        name: String::from("ID"), 
        kind: AttributeKind::General, 
        description: String::from("Unique identifier"), 
        is_mandatory: true 
    },
    Attribute { 
        key: String::from("content"), 
        name: String::from("Content"), 
        kind: AttributeKind::General, 
        description: String::from("Object contet"), 
        is_mandatory: true 
    },
    Attribute { 
        key: String::from("createdAt"), 
        name: String::from("Created at"), 
        kind: AttributeKind::DateTime, 
        description: String::from("Creation date"), 
        is_mandatory: true 
    },
    Attribute { 
        key: String::from("updatedAt"), 
        name: String::from("Updated at"), 
        kind: AttributeKind::DateTime, 
        description: String::from("Last update date"), 
        is_mandatory: true 
    },
    Attribute { 
        key: String::from("deletedAt"), 
        name: String::from("Deleted at"), 
        kind: AttributeKind::DateTime, 
        description: String::from("Deletion date"), 
        is_mandatory: false 
    },
    Attribute { 
        key: String::from("author"), 
        name: String::from("Author"), 
        kind: AttributeKind::String, 
        description: String::from("Author"), 
        is_mandatory: true 
    },
    Attribute { 
        key: String::from("indexParentId"), 
        name: String::from("Parent ID"), 
        kind: AttributeKind::Integer, 
        description: String::from("Upper Level"), 
        is_mandatory: false 
    },
    Attribute { 
        key: String::from("indexLevel"), 
        name: String::from("Index Level"), 
        kind: AttributeKind::Integer, 
        description: String::from("Hierarchy identifier"), 
        is_mandatory: false 
    },
    Attribute { 
        key: String::from("header"), 
        name: String::from("Header"), 
        kind: AttributeKind::String, 
        description: String::from("Content header"), 
        is_mandatory: false 
    },
]);
