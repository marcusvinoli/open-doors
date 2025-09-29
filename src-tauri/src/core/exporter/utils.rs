use crate::core::module::{Attribute, Object};

pub fn get_attribute_value(attribute: &Attribute, object: &Object) -> String {
	match attribute.key.as_str() {
		"id" => object.id().to_string(),
		"header" => object.header.clone(),
		"content" => object.content.clone(),
		"index_parent_id" => object.index_parent_id.to_string(),
		"index_level" => object.index_level.clone(),
		"author" => object.author.clone(),
		"created_at" => object.created_at.to_string(),
		"updated_at" => object.updated_at.to_string(),
		"deleted_at" => if object.deleted_at.is_none() { "".into() } else { object.deleted_at.unwrap().to_string() },
		_ => {
			if let Some(attributes) = &object.attributes {
				if let Some(value) = attributes.get(&attribute.key) {
					value.to_string()
				} else {
					String::new()
				}
			} else {
				String::new()
			}
		}
	}
}
