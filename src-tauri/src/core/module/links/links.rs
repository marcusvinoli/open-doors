use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use super::Link;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Links {
    pub inbound_links: HashMap<usize, Vec<Link>>,
    pub outbound_links: HashMap<usize, Vec<Link>>,
}
