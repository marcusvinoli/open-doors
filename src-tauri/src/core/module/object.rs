use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::core::user::User;

use super::links::Link;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Object {
    id: usize,
    pub level: String,
    pub header: String,
    pub content: String,
    pub author: User,
    pub is_active: bool,
    pub is_normative: bool,
    pub is_requirement: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub custom_fields: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub outbound_links: Option<Vec<Link>>,
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
