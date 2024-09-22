use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use csv::WriterBuilder;
use crate::core::module::{Module, object::Object};

#[cfg(windows)]
const LINE_ENDING : &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING : &'static str = "\n";

pub struct CsvOptions {
	default_view: bool,
	show_deleted: bool,
}

impl CsvOptions {
	pub fn builder() -> CsvOptionsBuilder {
		CsvOptionsBuilder::default()
	}
}

#[derive(Debug, Default)]
pub struct CsvOptionsBuilder {
	default_view: bool,
	show_deleted: bool,
}

impl CsvOptionsBuilder {
	pub fn default_view(mut self) -> Self {
		self.default_view = true;
		self
	}

	pub fn show_deleted(mut self, yes: bool) -> Self {
		self.show_deleted = yes;
		self
	}

	pub fn build(self) -> CsvOptions {
		CsvOptions { 
			default_view: self.default_view, 
			show_deleted: self.show_deleted,
		}
	}
}

#[derive(Serialize, Deserialize)]
struct CsvObjectInterface {
	#[serde(rename="Level")]
	level: String,
	#[serde(rename="ID")]
	id: String,
	#[serde(rename="Content")]
	content: String,
	#[serde(rename="Author")]
	author: String,
	#[serde(rename="Is Active?")]
	active: String,
	#[serde(rename="Is Normative?")]
	normative: String,
	#[serde(rename="Is Requirement?")]
	requirement: String,
}

impl CsvObjectInterface {
	pub fn from_objects(module: &Module, objects: Vec<Object>) -> Vec<CsvObjectInterface> {
		objects.into_iter().map(|obj| CsvObjectInterface::from_object(&module, obj)).collect()
	}

	pub fn from_object(module: &Module, object: Object) -> CsvObjectInterface {
		let id: String;
		let content: String;
		let active: String;
		let requirement: String;
		let normative: String;
		let author: String;
		let level: String;
		
		level = format!("{}", object.level);
		id = format!("{}{}{}", module.manifest.prefix, module.manifest.separator, object.id());
		active = if object.is_active {String::from("Yes")} else {String::from("No")};
		requirement = if object.is_requirement {String::from("Yes")} else {String::from("No")};
		normative = if object.is_normative {String::from("Yes")} else {String::from("No")};
		author = format!("{} <{}>", object.author.name, object.author.email);
		content = if object.header != String::from("") {
			format!("{}{}{}", object.header, LINE_ENDING, object.content)
		} else {
			object.content
		};

		CsvObjectInterface {
			id,
			level,
			content,
			active,
			requirement,
			normative,
			author,
		}
	}

}

pub struct CsvExporter {

}

impl CsvExporter {
	pub fn export<T: Into<String> + ?Sized>(path: &PathBuf, filename: T, module: &Module, objects: Vec<Object>, options: &CsvOptions) -> bool {
		let objects: Vec<Object> = if options.show_deleted {
			objects.into_iter().filter(|o| o.deleted_at.is_none()).collect()
		} else {
			objects
		};
		
		let file_path: PathBuf = path.join(Into::<String>::into(filename));

		let csv_int: Vec<CsvObjectInterface> = CsvObjectInterface::from_objects(module, objects);

		if let Ok(mut csv) = WriterBuilder::new()
			.flexible(true)
			.has_headers(true)
			.from_path(file_path) {
				let mut result: bool = true;
				csv_int.into_iter().for_each(|dt| {
					if csv.serialize(dt).is_ok() {
						result = result & true;
					} else {
						result = result & false;
					}
				});
				return result;
			}
		false
	}
}
