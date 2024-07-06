use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct Object {
    id: usize,
    level: String,
    header: String,
    content: String,
    author: String,
    is_active: bool,
    is_normative: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if="Option::is_none")]
    deleted_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if="Option::is_none")]
    custom_fields: Option<HashMap<String, String>>,
}
