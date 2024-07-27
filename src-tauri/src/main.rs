#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod handlers;
pub mod core;
pub mod git;

use crate::handlers as od_handlers;

fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
		od_handlers::repository::create_repo,
		od_handlers::repository::clone_repo,
		od_handlers::repository::read_repo,
		od_handlers::tree::read_tree,
		od_handlers::project::read_project,
		od_handlers::project::create_project,
		])
		.run(tauri::generate_context!())
		.expect("Error while running OpenDOORs.");
}
