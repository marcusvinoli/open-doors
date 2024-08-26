pub mod path_utils {
	use serde::{Serializer, Deserialize, Deserializer};
	use std::{borrow::Cow, path::PathBuf};

	pub fn serialize<S>(p: &PathBuf, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		// Convert the path to a Unix-style string
		let unix_style_path = p.to_str()
			.unwrap_or("")
			.replace(std::path::MAIN_SEPARATOR, "/");
		serializer.serialize_str(&unix_style_path)
	}
	
	pub fn deserialize<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: Cow<'de, str> = Deserialize::deserialize(deserializer)?;
		let path_str = s.replace("/", &std::path::MAIN_SEPARATOR.to_string());
		Ok(PathBuf::from(path_str))
	}
}