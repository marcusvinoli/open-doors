use std::path::PathBuf;

use tauri::command;

use crate::core::{error::OpenDoorsError, exporter::{csv::{CsvExporter, CsvOptions}, xlsx::{XlsxExporter, XlsxOptions}}, module::{Module, Object, View}};

#[command] 
pub fn export_csv(destination: PathBuf, file_name: String, module: Module, view: View, objects: Vec<Object>) -> Result<(), OpenDoorsError> {
	let options: CsvOptions = CsvOptions::builder().show_deleted(false).build();
	Ok(CsvExporter::export_view(&destination, &file_name, &module, &view, &objects, &options)?)
}

#[command]
pub fn export_xlsx(destination: PathBuf, file_name: String, module: Module, view: View, objects: Vec<Object>) -> Result<(), OpenDoorsError> {
	let options: XlsxOptions = XlsxOptions::builder().show_deleted(false).build();
	Ok(XlsxExporter::export_view(&destination, &file_name, &module, &view, &objects, &options)?)
}
