use serde::{Serialize, Deserialize};

use crate::core::module::Link;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum ObjectStatus {
    #[default]
    Draft,
    Baselined,
    Updated,
    Deleted,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Metadata {
    pub status: ObjectStatus,
    pub inbound_links: Option<Vec<Link>>,
    pub outbound_links: Option<Vec<Link>>,
}
