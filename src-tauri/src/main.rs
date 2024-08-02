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
		od_handlers::user::get_user,
		od_handlers::tree::read_tree,
		od_handlers::project::create_project,
		od_handlers::project::read_project,
		od_handlers::project::update_project,
		od_handlers::project::delete_project,
		od_handlers::folder::create_folder,
		od_handlers::folder::read_folder,
		od_handlers::folder::update_folder,
		od_handlers::folder::delete_folder,
		od_handlers::module::create_module,
		od_handlers::module::read_module,
		od_handlers::module::create_object,
		od_handlers::module::create_draft_object,
		od_handlers::module::read_object,
		od_handlers::module::read_draft_object,
		od_handlers::module::read_objects,
		od_handlers::module::read_draft_objects,
		od_handlers::module::update_object,
		od_handlers::module::update_draft_object,
		od_handlers::module::delete_object,
		od_handlers::module::create_template,
		od_handlers::module::read_template,
		od_handlers::module::update_template,
		])
		.run(tauri::generate_context!())
		.expect("Error while running OpenDOORS.");
}
