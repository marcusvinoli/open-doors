use std::{collections::HashMap, path::PathBuf};

use regex::Regex;
use xlsxwriter::{Format, Workbook, Worksheet, XlsxError};

use crate::core::module::{object::Object, template::{self, Template}, Module};

pub struct XlsxOptions {
	sheet_name: Option<String>,
	default_view: bool, // Reserved for future usage. 
	show_deleted: bool,
	rich_text: bool,
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
			rich_text: self.rich_text
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

		<Vec<template::Fields> as Clone>::clone(&template.fields).into_iter().for_each(|field| {
			has_error |= ws.write_string(0, col, &field.attribute, Some(Format::new().set_bold())).is_err();
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
				fmt = Format::new().set_bold().set_font_size(XlsxExporter::get_font_size_from_level(&object.level)).to_owned();
				Some(&fmt)
			};

			if object.header.is_empty() {
				ws.write_string(row, 1, &XlsxExporter::remove_markdown(&object.content), format.clone())?;
			} else {
				ws.write_string(row, 1, &XlsxExporter::remove_markdown(&object.header), format.clone())?;
			}

			ws.write_string(row, 0, &format!("{}{}{}", module.manifest.prefix, module.manifest.separator, object.id()), format.clone())?;
			ws.write_string(row, 2, &format!("{} <{}>", object.author.name, object.author.email), None)?;
			ws.write_string(row, 3, if object.is_active {"Yes"} else {"No"}, None)?;
			ws.write_string(row, 4, if object.is_normative {"Yes"} else {"No"}, None)?;
			ws.write_string(row, 5, if object.is_requirement {"Yes"} else {"No"}, None)?;

			<Vec<template::Fields> as Clone>::clone(&template.fields).into_iter().for_each(|field| {
				ws.write_string(row, col, 
					&<Option<HashMap<String, String>> as Clone>::clone(&object.custom_fields)
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

	fn get_font_size_from_level(level: &String) -> f64 {
		let levels: Vec<&str> = level.split(|s| s == '.' || s == '-').collect();
		return match level.len() {
			1 => { 16.0 },
			2 => { 14.0 },
			3 => { 12.0 },
			_ => { 11.0 },
		};
	}
}
