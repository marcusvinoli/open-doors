use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::core::user::User;

use super::Metadata;


#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Object {
    id: usize,
    pub parent_level: usize,
    pub index_level: usize,
    pub header: String,
    pub content: String,
    pub author: User,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_fields: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl Object {
    pub fn assign_id(&mut self, id: usize) {
        if self.id == 0 {
            self.id = id;
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
