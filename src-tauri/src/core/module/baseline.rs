use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::core::utils::SemVer;

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all="camelCase")]
pub enum BaselineStatus {
	#[default]
	WorkInProgress,
	Latest,
	Historical,
	Deleted,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Baseline {
	pub version: SemVer,
	pub description: String,
	pub status: BaselineStatus,
	pub created_at: DateTime<Utc>,
	pub created_by: String,
	#[serde(skip_serializing_if="Option::is_none")]
	pub deleted_at: Option<DateTime<Utc>>,
	#[serde(skip_serializing_if="Option::is_none")]
	pub deleted_by: Option<String>,
	#[serde(skip_serializing_if="Option::is_none")]
	pub hash: Option<String>,
}
