use std::{collections::HashMap, fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};
use csv::{Error as CsvError, Writer};

use crate::core::{exporter::{rich_text::RichText, utils::get_attribute_value}, module::{Attribute, AttributeKind, Module, Object, ObjectStatus, View, READ_ONLY_ATTRIBUTES}};

pub struct CsvOptions {
	show_deleted: bool,
}

impl CsvOptions {
	pub fn builder() -> CsvOptionsBuilder {
		CsvOptionsBuilder::default()
	}
}

#[derive(Debug, Default)]
pub struct CsvOptionsBuilder {
	show_deleted: bool,
	keep_markdown:bool,
}

impl CsvOptionsBuilder {
	pub fn show_deleted(mut self, yes: bool) -> Self {
		self.show_deleted = yes;
		self
	}

	pub fn keep_markdown(mut self, yes: bool) -> Self {
		self.keep_markdown = yes;
		self
	}

	pub fn build(self) -> CsvOptions {
		CsvOptions { 
			show_deleted: self.show_deleted,
		}
	}
}

#[derive(Serialize, Deserialize)]
struct CsvObjectInterface {
	id: String,
	content: String,
	author: String,
}

pub struct CsvExporter {}

impl CsvExporter {
	pub fn export_view(path: &PathBuf, filename: &String, module: &Module, view: &View, objects: &Vec<Object>, options: &CsvOptions) -> Result<(), CsvError> {
		let mut csv = Writer::from_path(&path.join(filename).to_string_lossy().to_string())?;
		let objects: Vec<Object> = objects.iter().filter(|obj| {
			if !options.show_deleted {
				if let Some(metadata) = &obj.metadata {
					if metadata.status == ObjectStatus::Deleted {
						return false;
					}
					return true;
				}
				return false;
			}
			true
		}).cloned().collect();
		let mut attribute_list: HashMap<String, Attribute> = HashMap::new();
		let mut attributes: Vec<Attribute> = Vec::new();

		for read_only_attribute in READ_ONLY_ATTRIBUTES.clone() {
			attribute_list.insert(read_only_attribute.key.to_string(), read_only_attribute.clone());
		}
		
		for custom_attribute in module.template.fields.clone() {
			attribute_list.insert(custom_attribute.key.to_string(), custom_attribute.clone());
		}

		for view_item in &view.items {
			if view_item.show {
				if let Some(attribute) = attribute_list.get(&view_item.key) {
					attributes.push((*attribute).clone());
				}
			}
		}
		Self::write_header(&mut csv, &attributes)?;
		Self::write_rows(&mut csv, &module, &attributes, &objects)?;
		csv.flush()?;
		
		Ok(())
	}
	
	fn write_header(csv: &mut Writer<File>, attributes: &Vec<Attribute>) -> Result<(), CsvError> {
		let mut header: Vec<String> = Vec::new();
		
		for attribute in attributes {
			header.push(attribute.name.to_string());
		}

		csv.write_record(header.as_slice())?;

		Ok(())
	}

	fn write_rows(csv: &mut Writer<File>, module: &Module, attributes: &Vec<Attribute>, objects: &Vec<Object>) -> Result<(), CsvError> {
		for object in objects {
			let mut row: Vec<String> = Vec::new();
			for attribute in attributes {
				row.push(Self::get_value(&module, &attribute, &object));
			}
			csv.write_record(row.as_slice())?;
			row.clear();
		}
		Ok(())
	}

	fn get_content(object: &Object) -> String {
		let mut content: String = String::new();
		if object.header.is_empty() && object.content.is_empty() {
			return content;
		}
		if !object.header.is_empty() {
			let mut level: String = "".into();
			if let Some(metadata) = &object.metadata {
				level = metadata.level.to_string();
			}
			let header: String = format!("{} {}", level, object.header);
			content.push_str(&header);
		}
		if !object.content.is_empty() {
			let text: String = RichText::parse(&object.content).clear_format();
			content.push_str(&text);
		}
		return content;
	}

	fn get_id(module: &Module, object: &Object) -> String {
		return format!("{}{}{}", module.manifest.prefix, module.manifest.separator, object.id());
	}

	fn get_value(module: &Module, attribute: &Attribute, object: &Object) -> String {
		if attribute.key == "id" {
			return Self::get_id(&module, &object);
		}
		if attribute.key == "content" {
			return Self::get_content(&object);
		}
		let value = get_attribute_value(&attribute, &object);
		if attribute.kind == AttributeKind::General {
			return RichText::parse(&value).clear_format();
		}
		return value;
	}
}
