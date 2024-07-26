pub mod error;

use std::{fs::{self, ReadDir, File}, io::{Read, Write}, path::PathBuf};
use error::MiddlewareError;
use serde::{Deserialize, Serialize};

pub fn create_folder(base_path: &PathBuf, folder_name: &String) -> Result<PathBuf, MiddlewareError> {
    let path = base_path.join(folder_name);
    fs::create_dir(&path)?;
    Ok(path)
}

pub fn read_folder(path: &PathBuf) -> Result<ReadDir, MiddlewareError> {
    Ok(fs::read_dir(path)?)
}

pub fn delete_folder(path: &PathBuf) -> Result<(), MiddlewareError> {
    Ok(fs::remove_dir_all(path)?)
}

pub fn create_yml_file<T: Serialize>(path: &PathBuf, file_name: String, data: T) -> Result<(), MiddlewareError> {
    let contents: String = serde_yaml::to_string(&data)?;
    File::create(&path.join(file_name))?.write_all(&mut contents.into_bytes())?;
    Ok(())
}

pub fn update_yml_folder<T: Serialize>(path: &PathBuf, file_name: String, data: T) -> Result<(), MiddlewareError> {
    create_yml_file(path, file_name, data)
}

pub fn read_yml_file<T: for<'a> Deserialize<'a>>(path: &PathBuf, file_name: String) -> Result<T, MiddlewareError> {
    let mut s: String = String::new();
    File::open(&path.join(file_name))?.read_to_string(&mut s)?;
    Ok(serde_yaml::from_str(&s)?)
}

pub fn delete_yml_file(path: &PathBuf, file_name: String) -> Result<(), MiddlewareError> {
    Ok(fs::remove_file(path)?)
}