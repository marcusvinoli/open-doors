use std::{collections::HashMap, fmt::format, path::PathBuf};

use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use regex::Regex;
use pulldown_cmark::{Parser, Event, Tag};
use xlsxwriter::{format, prelude::DateTime as XlsxDateTime, Format, Workbook, Worksheet, XlsxError};

use crate::core::{exporter::rich_text::{self, RichText}, module::{Attribute, AttributeKind, Module, Object, ObjectStatus, Template, View, ViewItem, READ_ONLY_ATTRIBUTES}};

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
	pub fn export_view(path: &PathBuf, filename: &String, module: &Module, view: &View, objects: &Vec<Object>, option: &XlsxOptions) -> Result<(), XlsxError> {
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
				if attribute.key == "content" {
					Self::write_content_cell(ws, row, col, object)?;
					col += 1;
					continue;
				}
				if attribute.key == "id" {
					Self::write_cell(ws, row, col, &attribute.kind, &format!("{}{}{}", module.manifest.prefix, module.manifest.separator, object.id()))?;
					col += 1;
					continue;
				}
				Self::write_cell(ws, row, col, &attribute.kind, &Self::get_attribute_value(&attribute, &object))?;
				col += 1;
			}
			row += 1;
			col = 0;
		}
		Ok(())
	}

	fn write_cell(ws: &mut Worksheet, row: u32, col: u16, kind: &AttributeKind, content: &String) -> Result<(), XlsxError> {
		if content.is_empty() {
			return Ok(());
		}
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
			},
			AttributeKind::General => {
				let segments = RichText::parse(&content).to_xlsxwriter_format();
				if segments.len() == 0 {
					return Ok(());
				}
				if segments.len() > 1 {
					let text = segments.iter().map(|(text, format)| (text.as_str(), format.as_ref())).collect::<Vec<_>>();
					ws.write_rich_string(row, col, &text.as_slice(), None)?;
					return  Ok(());
				} else {
					ws.write_string(row, col, &segments[0].0, segments[0].1.as_ref())?;
				}
			},
			_ => {
				ws.write_string(row, col, &content, None)?;
			}
		}
		Ok(())
	}

	fn write_content_cell(ws: &mut Worksheet, row: u32, col: u16, object: &Object) -> Result<(), XlsxError> {
		if object.header.is_empty() && object.content.is_empty() {
			return Ok(());
		}
		let mut content: Vec<(String, Option<Format>)> = Vec::new();
		if !object.header.is_empty() {
			let mut level: String = "".into();
			if let Some(metadata) = &object.metadata {
				level = metadata.level.to_string();
			}
			let header: String = format!("{} {}", level, object.header);
			let mut format = Format::new();
			format.set_bold().set_font_size(Self::get_font_size_from_level(&level));
			content.push((header, Some(format)));
		}
		if !object.content.is_empty() && !content.is_empty() {
			content.push(("\n\n".into(), None));
		}
		if !object.content.is_empty() {
			content.append(&mut RichText::parse(&object.content).to_xlsxwriter_format());
		}
		let text = content.iter().map(|(text, format)| (text.as_str(), format.as_ref())).collect::<Vec<_>>();
		if content.len() == 1 {
			ws.write_string(row, col, &text[0].0, text[0].1)?;
		} else {
			ws.write_rich_string(row, col, &text, None)?;
		}
		Ok(())
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
