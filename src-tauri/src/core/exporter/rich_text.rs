use pulldown_cmark::{Parser, Event, Tag};

use crate::core::exporter::xlsx::XlsxExporter;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct RichTextFormat {
	pub font_name: Option<String>,
	// font size / 100
	pub font_size: Option<u32>,
	pub bold: bool,
	pub italic: bool,
	pub underline: bool,
	pub num_format: Option<String>,
	pub unlocked: bool,
	pub hidden: bool,
	pub rotation: Option<i16>,
	pub text_wrap: bool,
	pub indent: Option<u8>,
	pub shrink: bool,
}

impl RichTextFormat {
	#[must_use]
	pub fn new() -> Self {
		Self::default()
	}

	pub fn set_bold(&mut self) -> &mut Self {
		self.bold = true;
		self
	}

	pub fn reset_bold(&mut self) -> &mut Self {
		self.bold = false;
		self
	}

	pub fn set_italic(&mut self) -> &mut Self {
		self.italic = true;
		self
	}

	pub fn reset_italic(&mut self) -> &mut Self {
		self.italic = false;
		self
	}

	pub fn set_underline(&mut self) -> &mut Self {
		self.underline = true;
		self
	}

	pub fn reset_underline(&mut self) -> &mut Self {
		self.underline = false;
		self
	}

	pub fn is_default(&self) -> bool {
		*self == RichTextFormat::default()
	}

	pub fn to_xlsxwriter_format(&self) -> Option<xlsxwriter::Format> {
		if self.is_default() {
			return None;
		}

		let mut format: xlsxwriter::Format = xlsxwriter::Format::new();

		format.set_text_wrap(); // Default behavior

		if self.bold { format.set_bold(); }

		if self.italic { format.set_italic(); }

		if self.underline { format.set_underline(xlsxwriter::format::FormatUnderline::Single); }

		Some(format)
	}
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd, Eq, Hash)]
pub struct RichText {
	segments: Vec<(String, RichTextFormat)>,
}

impl RichText {
	pub fn parse(text: &str) -> Self {
		let parser = Parser::new(text);

		let mut current_format: RichTextFormat = RichTextFormat::new();
		let mut current_text = String::new();

		let mut rich_string_segments: Vec<(String, RichTextFormat)> = Vec::new();

		for event in parser {
			match event {
				Event::Text(text) => {
					current_text.push_str(&text);
				}
				Event::Start(Tag::Strong) => {
					if !current_text.is_empty() {
						rich_string_segments.push((std::mem::take(&mut current_text), std::mem::take(&mut current_format)));
					}
					current_format.set_bold();
				}
				Event::End(Tag::Strong) => {
					if !current_text.is_empty() {
						rich_string_segments.push((std::mem::take(&mut current_text), std::mem::take(&mut current_format)));
					}
					current_format.reset_bold();
				}
				Event::Start(Tag::Emphasis) => {
					if !current_text.is_empty() {
						rich_string_segments.push((std::mem::take(&mut current_text), std::mem::take(&mut current_format)));
					}
					current_format.set_italic();
				}
				Event::End(Tag::Emphasis) => {
					if !current_text.is_empty() {
						rich_string_segments.push((std::mem::take(&mut current_text), std::mem::take(&mut current_format)));
					}
					current_format.reset_italic();
				}
				// TODO: Support other markdown tags
				_ => {}
			}
		}

		if !current_text.is_empty() {
			rich_string_segments.push((current_text, current_format));
		}

		let rich_string_refs: Vec<(String, RichTextFormat)> = rich_string_segments
			.iter()
			.map(|(text, format)| (text.to_string(), format.clone()))
			.collect();

		RichText {
			segments: rich_string_refs
		}
	}

	pub fn to_xlsxwriter_format(&self) -> Vec<(String, Option<xlsxwriter::Format>)> {
		let mut result: Vec<(String, Option<xlsxwriter::Format>)> = Vec::new();

		if self.segments.is_empty() {
			return result;
		}

		for segment in self.segments.iter() {
			result.push((segment.0.to_owned(), segment.1.to_xlsxwriter_format()));
		}

		result
	}
}
