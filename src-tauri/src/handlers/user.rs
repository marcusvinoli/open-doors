use std::path::PathBuf;
use tauri::command; 

use crate::{core::{error::OpenDoorsError, user::User}, git};

#[command] 
pub fn get_user(path: PathBuf) -> Result<User, OpenDoorsError> {
    let mut user: User = User { name: String::new(), email: String::new() };
    user.name = git::get_user_name(path.to_str().unwrap())?;
    user.email = git::get_user_email(path.to_str().unwrap())?;
    
    Ok(user)
}
