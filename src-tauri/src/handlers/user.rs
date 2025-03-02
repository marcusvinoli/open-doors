use std::{path::PathBuf, sync::Mutex};

use git2::{Repository as GitRepository, Signature};
use tauri::{command, State}; 

use crate::core::{error::OpenDoorsError, user::User};

#[command] 
pub fn get_user(state: State<'_, Mutex<Option<GitRepository>>>, path: PathBuf) -> Result<User, OpenDoorsError> {
    let mut user: User = User { name: String::new(), email: String::new() };
    if let Some(ref repo) = *state.lock().unwrap() {
        let sig: Signature<'_> = repo.signature()?;
        user.name = sig.name().unwrap_or_default().into();
        user.email = sig.email().unwrap_or_default().into();
    }
    Ok(user)
}
