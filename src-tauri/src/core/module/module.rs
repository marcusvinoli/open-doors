use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::core::{error::ModuleError, middleware as mid};

use super::{baseline::Baseline, object::Object, template::Template, definitions as defs};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModuleManifest {
    pub title: String,
    pub prefix: String,
    pub separator: String,
    pub description: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Module{
    pub path: PathBuf,
    pub manifest: ModuleManifest,
    pub template: Template,
    pub baselines: Vec<Baseline>,
}

impl Module {
    pub fn create(path: &PathBuf, man: &ModuleManifest) -> Result<Module, ModuleError> {
        let module_path = mid::create_folder(&path, &man.prefix)?;
        
        mid::create_yml_file(&module_path, defs::OD_MODULE_MANIFEST_FILE_NAME, &man)?;
        mid::create_yml_file(&module_path, defs::OD_BASELINE_FILE_NAME, &())?;
        mid::create_yml_file(&module_path, defs::OD_TEMPLATE_FILE_NAME, &())?;
        mid::create_yml_file(&module_path, defs::OD_LINKS_FILE_NAME, &())?;

        mid::create_folder(&module_path, defs::OD_OBJS_FOLDER_NAME)?;
        mid::create_folder(&module_path, defs::OD_DRAFT_FOLDER_NAME)?;
        mid::create_folder(&module_path, defs::OD_ASSETS_FOLDER_NAME)?;

        Ok(Module { 
            path: module_path, 
            manifest: man.clone(), 
            template: Template::default(), 
            baselines: Vec::new(), 
        })
    }
    
    pub fn read(path: &PathBuf) -> Result<Module, ModuleError> {
        Module::check_for_module_folder(&path)?;
        let manifest: ModuleManifest = mid::read_yml_file(&path, defs::OD_MODULE_MANIFEST_FILE_NAME)?;
        let template: Template = mid::read_yml_file(&path, defs::OD_TEMPLATE_FILE_NAME)?;
        let baselines: Vec<Baseline> = mid::read_yml_file(&path, defs::OD_BASELINE_FILE_NAME)?;
        
        Ok(Module { 
            path: path.clone(), 
            manifest, 
            template, 
            baselines, 
        })
    }
    
    pub fn update(path: &PathBuf, man: &ModuleManifest) -> Result<ModuleManifest, ModuleError> {
        Module::check_for_module_folder(&path)?;

        mid::update_yml_folder(&path, defs::OD_MODULE_MANIFEST_FILE_NAME, &man)?;
        
        Ok(mid::read_yml_file::<ModuleManifest, _>(&path, defs::OD_MODULE_MANIFEST_FILE_NAME)?)
    }
    
    pub fn delete(path: &PathBuf) -> Result<(), ModuleError> {
        Module::check_for_module_folder(&path)?;
        Ok(mid::delete_folder(&path)?)
    }

    pub fn create_objects(path: &PathBuf, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
        todo!()
    }

    pub fn read_objects(path: &PathBuf, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
        todo!()
    }
    
    pub fn update_objects(path: &PathBuf, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
        todo!()
    }
    
    pub fn delete_objects(path: &PathBuf, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
        todo!()
    }

    pub fn create_asset(path: &PathBuf, asset: &PathBuf) -> Result<(), ModuleError> {
        todo!()
    }
    
    pub fn remove_asset(path: &PathBuf, asset: &PathBuf) -> Result<(), ModuleError> {
        todo!()
    }

    pub fn update_asset(path: &PathBuf, asset: &PathBuf) -> Result<(), ModuleError> {
        todo!()
    }

    pub fn delete_asset(path: &PathBuf, asset: &PathBuf) -> Result<(), ModuleError> {
        todo!()
    }
    
    pub fn create_draft_objects(path: &PathBuf, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
        todo!()
    }

    pub fn read_draft_objects(path: &PathBuf, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
        todo!()
    }
    
    pub fn update_draft_objects(path: &PathBuf, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
        todo!()
    }
    
    pub fn delete_draft_objects(path: &PathBuf, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
        todo!()
    }

    pub fn create_template(path: &PathBuf, template: Template) -> Result<(), ModuleError> {
        todo!()
    }
    
    pub fn read_template(path: &PathBuf) -> Result<Template, ModuleError> {
        todo!()
    }
    
    pub fn update_template(path: &PathBuf, template: Template) -> Result<Template, ModuleError> {
        todo!()
    }

    pub fn delete_template(path: &PathBuf) -> Result<(), ModuleError> {
        todo!()
    }

    /* pub fn add_link(path: &PathBuf, link: Link) -> Result<(), ModuleError> {
        todo!()
    } */

    /* pub fn remove_link(path: &PathBuf, link: Link) -> Result<(), ModuleError> {
        todo!()
    } */

    pub fn create_baseline(path: &PathBuf) -> Result<Vec<Baseline>, ModuleError> {
        todo!()
    }

    pub fn read_baselines(path: &PathBuf) -> Result<Vec<Baseline>, ModuleError> {
        todo!()
    }

    pub fn read_from_baseline(path: &PathBuf, baseline: Baseline) -> Result<Vec<Object>, ModuleError> {
        todo!()
    }

    fn check_for_module_folder(path: &PathBuf) -> Result<(), ModuleError> {
        if !mid::file_exists(&path.join(defs::OD_MODULE_MANIFEST_FILE_NAME))? {
            return Err(ModuleError::InvalidModuleDirectory)
        }
        Ok(())
    }
}
