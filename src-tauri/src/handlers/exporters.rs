use std::path::PathBuf;

use tauri::command;

use crate::core::{error::OpenDoorsError, exporter::csv::{CsvExporter, CsvOptions}, module::{object::Object, Module}};

#[command] 
pub fn export_csv(module_path: PathBuf, file_path: PathBuf) -> Result<bool, OpenDoorsError> {
	let mut module: Module = Module::read(&module_path)?;
	let objects: Vec<Object> = module.read_objects()?;
	let file_name: String = format!("{}_{}.csv", module.manifest.prefix, module.baselines.pop().unwrap_or_default().version.to_string());
	let options: CsvOptions = CsvOptions::builder().default_view().show_deleted(false).build();
	return Ok(CsvExporter::export(&file_path, &file_name, &module, objects, &options))
}
