use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

use crate::core::{user::User, Link, Links};

use super::{Metadata, ObjectStatus};


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
    pub attributes: Option<HashMap<String, String>>,
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

    pub fn add_metadata(&mut self, status: &ObjectStatus, links: &Links) {
        if let Some(links) = links.inbound_links.get(&self.id) {
            self.add_inbound_links(Some(links.to_owned()));
        }
        if let Some(links) = links.outbound_links.get(&self.id) {
            self.add_outbound_links(Some(links.to_owned()));
        }
        self.set_status(status.to_owned());
    }

    pub fn set_status(&mut self, status: ObjectStatus) {
        if let Some(meta) = self.metadata.as_mut() {
            meta.status = status.to_owned();
            self.metadata = Some(meta.to_owned());
        } else {
            let mut metadata: Metadata = Metadata::default();
            metadata.status = status.to_owned();
        }
    }

    pub fn add_inbound_links(&mut self, links: Option<Vec<Link>>) {
        if let Some(meta) = self.metadata.as_mut() {
            meta.inbound_links = links;
            self.metadata = Some(meta.to_owned());
        } else {
            let mut metadata: Metadata = Metadata::default();
            metadata.inbound_links = links;
            self.metadata = Some(metadata);
        }
    }

    pub fn add_outbound_links(&mut self, links: Option<Vec<Link>>) {
        if let Some(meta) = self.metadata.as_mut() {
            meta.outbound_links = links;
            self.metadata = Some(meta.to_owned());
        } else {
            let mut metadata: Metadata = Metadata::default();
            metadata.outbound_links = links;
            self.metadata = Some(metadata);
        }
    }

    pub fn delete_metadata(&mut self) {
        self.metadata = None;
    }
}
