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
		od_handlers::project::create_project,
		od_handlers::project::read_project,
		od_handlers::project::update_project,
		od_handlers::project::delete_project,
		od_handlers::folder::create_folder,
		od_handlers::folder::read_folder,
		od_handlers::folder::update_folder,
		od_handlers::folder::delete_folder,
		])
		.run(tauri::generate_context!())
		.expect("Error while running OpenDOORs.");
}
