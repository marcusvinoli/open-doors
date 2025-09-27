use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use regex::Regex;
use pulldown_cmark::{Parser, Event, Tag};
use xlsxwriter::{format, prelude::DateTime as XlsxDateTime, Format, Workbook, Worksheet, XlsxError};

use crate::core::module::{Attribute, AttributeKind, READ_ONLY_ATTRIBUTES, Module, Object, ObjectStatus, Template, View, ViewItem};

pub struct XlsxOptions {
	sheet_name: Option<String>,
	default_view: bool, // Reserved for future usage. 
	show_deleted: bool,
	formatted: bool,
}

impl XlsxOptions {
	pub fn builder() -> XlsxOptionsBuilder {
		XlsxOptionsBuilder::default()
	}
}

#[derive(Debug, Default)]
pub struct XlsxOptionsBuilder {
	sheet_name: Option<String>,
	show_deleted: bool,
	default_view: bool,
	rich_text: bool,
}

impl XlsxOptionsBuilder {
	pub fn sheet_name(mut self, name: impl Into<String>) -> Self {
		self.sheet_name = Some(name.into());
		self
	}

	pub fn show_deleted(mut self, yes: bool) -> Self {
		self.show_deleted = yes;
		self
	}

	pub fn rich_text(mut self, yes: bool) -> Self {
		self.rich_text = yes;
		self
	}

	pub fn default_view(mut self, yes: bool) -> Self {
		self.default_view = yes;
		self
	}

	pub fn build(self) -> XlsxOptions {
		XlsxOptions {
			sheet_name: self.sheet_name,
			default_view: self.default_view,
			show_deleted: self.show_deleted, 
			formatted: self.rich_text
		}
	}
}

pub struct XlsxExporter {
	file: PathBuf,
	options: XlsxOptions,
}

impl XlsxExporter {
	pub fn export(path: &PathBuf, filename: &String, module: &Module, objects: &Vec<Object>, option: &XlsxOptions) -> Result<(), XlsxError> {
		let wb: Workbook = Workbook::new(&path.join(filename).to_string_lossy())?;
		let mut ws: Worksheet = wb.add_worksheet(None)?;
		XlsxExporter::write_header(&mut ws, &module.template)?;
		XlsxExporter::write_content(&mut ws, &module, &module.template, &objects)?;
		wb.close()?;
		Ok(())
	}

	pub fn _export_view(path: &PathBuf, filename: &String, module: &Module, view: &View, objects: &Vec<Object>, option: &XlsxOptions) -> Result<(), XlsxError> {
		let wb: Workbook = Workbook::new(&path.join(filename).to_string_lossy())?;
		let mut ws: Worksheet = wb.add_worksheet(None)?;
		let objects: Vec<Object> = objects.iter().filter(|obj| {
			if !option.show_deleted {
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
		
		let mut attribute_list: HashMap<String, &Attribute> = HashMap::new();
		let mut attributes: Vec<Attribute> = Vec::new();
		
		let _ = READ_ONLY_ATTRIBUTES.iter().map(|attribute| attribute_list.insert(attribute.key.clone(), attribute));
		let _ = module.template.fields.iter().map(|attribute| attribute_list.insert(attribute.key.clone(), attribute));
		
		for view_item in &view.items {
			if view_item.show {
				if let Some(attribute) = attribute_list.get(&view_item.key) {
					attributes.push((*attribute).clone());
				}
			}
		}

		XlsxExporter::write_headers(&mut ws, &attributes)?;
		XlsxExporter::write_rows(&mut ws, &module ,&attributes, &objects)?;
		
		wb.close()?;
		Ok(())
	}

	fn write_headers(ws: &mut Worksheet, attributes: &Vec<Attribute>) -> Result<(), XlsxError> {
		let mut col = 0;
		for attribute in attributes {
			ws.write_string(0, col, &attribute.name, 
				Some(Format::new()
				.set_bold()
				.set_border_bottom(
					format::FormatBorder::Thin
				)))?;
			col += 1;
		}
		Ok(())
	}

	fn write_rows(ws: &mut Worksheet, module: &Module, attributes: &Vec<Attribute>, objects: &Vec<Object>) -> Result<(), XlsxError> {
		let mut row: u32 = 1;
		let mut col: u16 = 0;
		for object in objects {
			for attribute in attributes {
				let mut content: String = String::new();
				let content: String = XlsxExporter::get_attribute_value(&attribute, &object);

				XlsxExporter::write_cell(ws, row, col, &attribute.kind, &content)?;
				col += 1;
			}
			row += 1;
			col = 0;
		}
		Ok(())
	}

	fn write_cell(ws: &mut Worksheet, row: u32, col: u16, kind: &AttributeKind, content: &String) -> Result<(), XlsxError> {
		match kind {
			AttributeKind::Integer => {
				if let Ok(number) = content.parse::<f64>() {
					ws.write_number(row, col, number, None)?;
				} else {
					ws.write_string(row, col, &content, None)?;
				}
			},
			AttributeKind::DateTime => {
				if let Ok(datetime) =  DateTime::parse_from_rfc3339(&content) {
					let datetime = XlsxDateTime::new(
						datetime.year() as i16, 
						datetime.month() as i8, 
						datetime.day() as i8,
						datetime.hour() as i8,
						datetime.minute() as i8,
						datetime.second() as f64
					); 
					ws.write_datetime(row, col, &datetime, Some(&Format::new().set_num_format("mmm d yyyy hh:mm AM/PM")))?;
				} else {
					ws.write_string(row, col, &content, None)?;
				}
			}
			_ => {
				ws.write_string(row, col, &content, None)?;
			}
		}
		Ok(())
	}

	fn write_content_cell(ws: &mut Worksheet, row: u32, col: u16, object: &Object) -> Result<(), XlsxError> {

		Ok(())
	}

	fn to_rich_string(content: &String) -> Vec<(String, Format)> {
		let parser = Parser::new(content);

		let mut current_format: Option<Format> = None;
		let mut current_text = String::new();

		let mut rich_string_segments: Vec<(&Format, String)> = Vec::new();

		for event in parser {
			match event {
				Event::Text(text) => {
					current_text.push_str(&text);
				}
				Event::Start(Tag::Strong) => {
					if !current_text.is_empty() {
						rich_string_segments.push((current_format, std::mem::take(&mut current_text)));
					}
					current_format = &format_bold;
				}
				Event::End(Tag::Strong) => {
					if !current_text.is_empty() {
						rich_string_segments.push((current_format, std::mem::take(&mut current_text)));
					}
					current_format = &format_normal;
				}
				Event::Start(Tag::Emphasis) => {
					if !current_text.is_empty() {
						rich_string_segments.push((current_format, std::mem::take(&mut current_text)));
					}
					current_format = &format_italic;
				}
				Event::End(Tag::Emphasis) => {
					if !current_text.is_empty() {
						rich_string_segments.push((current_format, std::mem::take(&mut current_text)));
					}
					current_format = &format_normal;
				}
				// Ignora outros eventos (parágrafos, cabeçalhos, etc.)
				_ => {}
			}
		}

		if !current_text.is_empty() {
			rich_string_segments.push((current_format, current_text));
		}

		let rich_string_refs: Vec<(String, Format)> = rich_string_segments
			.iter()
			.map(|(format, text)| (text.to_string(), *format.clone()))
			.collect();

		return rich_string_refs;
	}

	fn get_attribute_value(attribute: &Attribute, object: &Object) -> String {
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

	fn write_header(ws: &mut Worksheet, template: &Template) -> Result<(), XlsxError> {
		let mut binding: Format = Format::new();
  		let bold_fmt = binding.set_bold();
		ws.write_string(0, 0, "ID", Some(&bold_fmt))?;
		ws.write_string(0, 1, "Object Text", Some(&bold_fmt))?;
		ws.write_string(0, 2, "Author", Some(&bold_fmt))?;
		ws.write_string(0, 3, "Is Active", Some(&bold_fmt))?;
		ws.write_string(0, 4, "Is Normative", Some(&bold_fmt))?;
		ws.write_string(0, 5, "Is Requirement", Some(&bold_fmt))?;
		
		let mut col: u16 = 6;
		let mut has_error: bool = false;

		<Vec<Attribute> as Clone>::clone(&template.fields).into_iter().for_each(|field| {
			has_error |= ws.write_string(0, col, &field.name, Some(Format::new().set_bold())).is_err();
			col += 1;
		});

		Ok(())
	}

	fn write_content(ws: &mut Worksheet, module: &Module, template: &Template, objects: &Vec<Object>) -> Result<(), XlsxError> {
		let mut col: u16 = 6;
		let mut row: u32 = 1;

		objects.into_iter().try_for_each(|object| -> Result<(), XlsxError> {
			let fmt: Format;
			let format: Option<&Format> = if object.header.is_empty() {
				None
			} else {
				// fmt = Format::new().set_bold().set_font_size(XlsxExporter::get_font_size_from_level(&object.level)).to_owned();
				fmt = Format::new();
				Some(&fmt)
			};

			if object.header.is_empty() {
				ws.write_string(row, 1, &XlsxExporter::remove_markdown(&object.content), format.clone())?;
			} else {
				ws.write_string(row, 1, &XlsxExporter::remove_markdown(&object.header), format.clone())?;
			}

			ws.write_string(row, 0, &format!("{}{}{}", module.manifest.prefix, module.manifest.separator, object.id()), format.clone())?;
			ws.write_string(row, 2, &object.author, None)?;

			<Vec<Attribute> as Clone>::clone(&template.fields).into_iter().for_each(|field| {
				ws.write_string(row, col, 
					&<Option<HashMap<String, String>> as Clone>::clone(&object.attributes)
						.unwrap_or_default()
						.get(&field.key)
						.unwrap_or(&String::new()), 
					None).unwrap_or_default();
				col += 1;
			});

			col = 6;
			row += 1;

			Ok(())
		})?;

		Ok(())
	}

	fn remove_markdown(input: &str) -> String {
		let re_bold = Regex::new(r"\*\*(.*?)\*\*").unwrap();     	// Bold: **text**
		let re_italic = Regex::new(r"\*(.*?)\*").unwrap();        	// Italic: *text*
		let re_italic_underline = Regex::new(r"_(.*?)_").unwrap();	// Underscore Underline: _text_
		let re_header = Regex::new(r"#+\s*(.*)").unwrap();        	// Header: # Headers
		let re_links = Regex::new(r"\[.*?\]\(.*?\)").unwrap();    	// Links: [text](link)
		let re_inline_code = Regex::new(r"`(.*?)`").unwrap();     	// Code: `code`
	
		let result = re_bold.replace_all(input, "$1");
		let result = re_italic.replace_all(&result, "$1");
		let result = re_italic_underline.replace_all(&result, "$1");
		let result = re_header.replace_all(&result, "$1");
		let result = re_links.replace_all(&result, "$1 ($2)");
		let result = re_inline_code.replace_all(&result, "$1");

		result.to_string()
	}

	fn get_font_size_from_level(level: &String) -> f64 {
		let levels: Vec<&str> = level.split(|s| s == '.' || s == '-').collect();
		return match levels.len() {
			1 => { 16.0 },
			2 => { 14.0 },
			3 => { 12.0 },
			_ => { 11.0 },
		};
	}
}
