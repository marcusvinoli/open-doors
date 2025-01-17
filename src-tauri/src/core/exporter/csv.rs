use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use csv::WriterBuilder;
use regex::Regex;

use crate::core::module::{Module, object::Object};

#[cfg(windows)]
const LINE_ENDING : &str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING : &str = "\n";

pub struct CsvOptions {
	default_view: bool, // Reserved for future usage. 
	show_deleted: bool,
	keep_markdown: bool,
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
	keep_markdown:bool,
}

impl CsvOptionsBuilder {
	pub fn default_view(mut self, yes: bool) -> Self {
		self.default_view = yes;
		self
	}

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
			default_view: self.default_view, 
			show_deleted: self.show_deleted,
			keep_markdown: self.keep_markdown,
		}
	}
}

#[derive(Serialize, Deserialize)]
struct CsvObjectInterface {
	#[serde(rename="Level")]
	level: String,
	#[serde(rename="ID")]
	id: String,
	#[serde(rename="Object Text")]
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
	pub fn from_objects(module: &Module, objects: Vec<Object>, keep_markdown: bool) -> Vec<CsvObjectInterface> {
		objects.into_iter().map(|obj| CsvObjectInterface::from_object(&module, obj, keep_markdown)).collect()
	}

	pub fn from_object(module: &Module, object: Object, keep_markdown: bool) -> CsvObjectInterface {
		let id: String;
		let mut content: String;
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

		if !keep_markdown {
			content = CsvObjectInterface::remove_markdown(&content);
		} 

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

	fn remove_markdown(input: &str) -> String {
		let re_bold = Regex::new(r"\*\*(.*?)\*\*").unwrap();     // Bold: **text**
		let re_italic = Regex::new(r"\*(.*?)\*").unwrap();        // Italic: *text*
		let re_italic_underline = Regex::new(r"_(.*?)_").unwrap();     // Underscore Underline: _text_
		let re_header = Regex::new(r"#+\s*(.*)").unwrap();        // Header: # Headers
		let re_links = Regex::new(r"\[.*?\]\(.*?\)").unwrap();    // Links: [text](link)
		let re_inline_code = Regex::new(r"`(.*?)`").unwrap();     // Code: `code`
	
		let result = re_bold.replace_all(input, "$1");
		let result = re_italic.replace_all(&result, "$1");
		let result = re_italic_underline.replace_all(&result, "$1");
		let result = re_header.replace_all(&result, "$1");
		let result = re_links.replace_all(&result, "$1 ($2)");
		let result = re_inline_code.replace_all(&result, "$1");

		result.to_string()
	}
}

pub struct CsvExporter {

}

impl CsvExporter {
	pub fn export<T: Into<String> + ?Sized>(path: &PathBuf, filename: T, module: &Module, objects: Vec<Object>, options: &CsvOptions) -> bool {
		let objects: Vec<Object> = if options.show_deleted {
			objects
		} else {
			objects.into_iter().filter(|o| o.deleted_at.is_none()).collect()
		};
		
		let file_path: PathBuf = path.join(Into::<String>::into(filename));

		let csv_int: Vec<CsvObjectInterface> = CsvObjectInterface::from_objects(module, objects, options.keep_markdown);

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
