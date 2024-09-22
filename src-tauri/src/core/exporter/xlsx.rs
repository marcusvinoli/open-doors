use std::{option, path::PathBuf};

use serde::Deserialize;

use crate::core::module::{object::Object, Module};

struct XlsxOptions {
	sheet_name: Option<String>,
	include_deleted: bool,
	rich_text: bool,
}

impl XlsxOptions {
	fn builder() -> XlsxOptionsBuilder {
		XlsxOptionsBuilder::default()
	}
}

#[derive(Debug, Default)]
struct XlsxOptionsBuilder {
	sheet_name: Option<String>,
	include_deleted: bool,
	rich_text: bool,
}

impl XlsxOptionsBuilder {
	pub fn sheet_name(mut self, name: impl Into<String>) -> Self {
		self.sheet_name = Some(name.into());
		self
	}

	pub fn include_deleted_objects(mut self, inc_deleted: bool) -> Self {
		self.include_deleted = inc_deleted;
		self
	}

	pub fn rich_text(mut self, rich_text: bool) -> Self {
		self.rich_text = rich_text;
		self
	}

	fn build(self) -> XlsxOptions {
		XlsxOptions {
			sheet_name: self.sheet_name,
			include_deleted: self.include_deleted, 
			rich_text: self.rich_text
		}
	}
}


struct XlsxExporter {
	file: PathBuf,
	options: XlsxOptions,
}

impl XlsxExporter {
	pub fn export(path: &PathBuf, module: &Module, objects: &Vec<Object>, option: &XlsxOptions) {
		
	}
}
