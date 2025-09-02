use serde::{Serialize, Deserialize};

use crate::core::module::{Link, ObjectStatus};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Metadata {
    pub level: String,
    pub status: ObjectStatus,
    pub inbound_links: Option<Vec<Link>>,
    pub outbound_links: Option<Vec<Link>>,
}
