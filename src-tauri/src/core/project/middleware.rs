use std::io::{Read, Write};
use std::fs::{self, File};
use std::path::PathBuf;

use crate::core::RepositoryError;
use super::project::ProjectManifest;
use super::definitions::OD_PROJECT_MANIFEST_FILE_NAME;

pub fn create_project_manifest(man: &ProjectManifest) -> std::io::Result<()> {
    let manifest_content: String = serde_yaml::to_string(man).unwrap();
    let mut loc: PathBuf = man.location.clone();
    loc.push(OD_PROJECT_MANIFEST_FILE_NAME);
    File::create(loc)?.write_all(&mut manifest_content.into_bytes())?;
    Ok(())
}

pub fn read_project_manifest(repo_path: &PathBuf) -> Result<ProjectManifest, RepositoryError> {
    let mut file_content: String = String::new();
    let mut loc: PathBuf = repo_path.clone();
    loc.push(OD_PROJECT_MANIFEST_FILE_NAME);
    File::open(loc)?.read_to_string(&mut file_content)?;
    let man: ProjectManifest = serde_yaml::from_str(&file_content)?;
    Ok(man)
}

pub fn update_project_manifest(man: &ProjectManifest) -> std::io::Result<()> {
    create_project_manifest(man)
}

pub fn delete_project_manifest(man: ProjectManifest) -> std::io::Result<()> {
    fs::remove_dir_all(&man.location)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn od_manifest_creation() {
        let man: ProjectManifest = ProjectManifest{name: "Projet Test".into(), location: "./test/test1".into(), prefix: "PRJ".into(), separator: "".into()};
        assert!(create_project_manifest(&man).is_ok());
    }
    
    #[test]
    fn od_manifest_reading() {
        let man: ProjectManifest = ProjectManifest{name: "Projet Test 1".into(), location: "./test/test2".into(), prefix: "PRJ".into(), separator: "".into()};
        assert!(create_project_manifest(&man).is_ok());
        
        let res = read_project_manifest(&PathBuf::from("./test/test2"));
        assert!(res.is_ok());
        
        if let Ok(res) = res {
            assert_eq!(res.name, String::from("Projet Test 1"));
            assert_eq!(res.location, PathBuf::from("./test/test2"));
            assert_eq!(res.prefix, String::from("PRJ"));
        }

    }

    #[test]
    fn od_manifest_updating() {
        let mut man: ProjectManifest = ProjectManifest{name: "Project Test".into(), location: "./test/test3".into(), prefix: "PRJ".into(), separator: "".into()};
        assert!(create_project_manifest(&man).is_ok());
        
        let res = read_project_manifest(&PathBuf::from("./test/test3"));
        assert!(res.is_ok());

        if let Ok(res) = res {
            assert_eq!(res.name, String::from("Project Test"));
            assert_eq!(res.location, PathBuf::from("./test/test3"));
            assert_eq!(res.prefix, String::from("PRJ"));
            man = res;
        }
        
        man.name = String::from("Hello World");

        assert!(update_project_manifest(&man).is_ok());

        let res = read_project_manifest(&PathBuf::from("./test/test3"));
        assert!(res.is_ok());

        if let Ok(res) = res {
            assert_eq!(res.name, String::from("Hello World"));
            assert_eq!(res.location, PathBuf::from("./test/test3"));
        }

    }
}
