use std::{cmp::max, path::PathBuf, vec};

use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::core::{error::ModuleError, middleware as mid};

use super::{baseline::Baseline, definitions as defs, object::Object, template::Template};

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
        let baselines: Vec<Baseline> = vec![Baseline::default()];
        let template: Template = Template::default();
        
        mid::create_yml_file(&module_path, defs::OD_MODULE_MANIFEST_FILE_NAME, &man)?;
        mid::create_yml_file(&module_path, defs::OD_BASELINE_FILE_NAME, &baselines)?;
        mid::create_yml_file(&module_path, defs::OD_TEMPLATE_FILE_NAME, &template)?;
        mid::create_yml_file(&module_path, defs::OD_LINKS_FILE_NAME, &())?;

        mid::create_folder(&module_path, defs::OD_OBJS_FOLDER_NAME)?;
        mid::create_folder(&module_path, defs::OD_DRAFT_FOLDER_NAME)?;
        mid::create_folder(&module_path, defs::OD_ASSETS_FOLDER_NAME)?;

        Ok(Module { 
            path: module_path, 
            manifest: man.clone(), 
            template,
            baselines,
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

    pub fn read_object(&self, id: usize) -> Result<Object, ModuleError> {
        Ok(Module::open_object(&self.path.join(defs::OD_OBJS_FOLDER_NAME), id)?)
    }

    pub fn read_draft_object(&self, id: usize) -> Result<Object, ModuleError> {
        Ok(Module::open_object(&self.path.join(defs::OD_OBJS_FOLDER_NAME), id)?)
    }

    pub fn find_object(&self, id: usize) -> Result<Object, ModuleError> {
        match Module::open_object(&self.path.join(defs::OD_OBJS_FOLDER_NAME), id) {
            Ok(obj) => {
                return Ok(obj)
            },
            Err(_) => {
                return Module::open_object(&self.path.join(defs::OD_DRAFT_FOLDER_NAME), id)
            }
        };
    }
    
    pub fn create_object(&mut self, obj: &mut Object) -> Result<Object, ModuleError> {
        let id: usize = self.save_object(defs::OD_DRAFT_FOLDER_NAME, obj)?;
        mid::move_file(&self.path.join(defs::OD_DRAFT_FOLDER_NAME), &self.path.join(defs::OD_OBJS_FOLDER_NAME), &format!("{id}.yml"))?;
        Ok(self.find_object(id)?)
    }
    
    pub fn create_draft_object(&mut self, obj: &mut Object) -> Result<Object, ModuleError> {
        let id: usize = self.save_object(defs::OD_DRAFT_FOLDER_NAME, obj)?;
        Ok(self.find_object(id)?)
    }

    pub fn create_objects(&mut self, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
        let mut res: Vec<Object> = Vec::new();
        for obj in objs {
            let mut obj = obj.clone();
            res.push(self.create_object(&mut obj)?);
        }
        Ok(res)
    }
    
    pub fn create_draft_objects(&mut self, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
        let mut res: Vec<Object> = Vec::new();
        for obj in objs {
            let mut obj = obj.clone();
            res.push(self.create_draft_object(&mut obj)?);
        }
        Ok(res)
    }
    
    pub fn read_objects(&mut self) -> Result<Vec<Object>, ModuleError> {
        let mut objs: Vec<Object> = Vec::new();

        for entry in mid::read_folder(&self.path.join(defs::OD_OBJS_FOLDER_NAME))? {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_str().unwrap_or("");
        
                if !file_name_str.ends_with(".yml") {
                    continue;
                }
    
                if let Some(number_str) = file_name_str.strip_suffix(".yml") {
                    if let Ok(number) = number_str.parse::<usize>() {
                        objs.push(self.find_object(number)?)
                    }
                }
                
            } else {
                continue;
            }
        }

        Ok(objs)
    }

    pub fn read_draft_objects(&mut self) -> Result<Vec<Object>, ModuleError> {
        let mut objs: Vec<Object> = Vec::new();

        for entry in mid::read_folder(&self.path.join(defs::OD_DRAFT_FOLDER_NAME))? {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_str().unwrap_or("");
        
                if !file_name_str.ends_with(".yml") {
                    continue;
                }
    
                if let Some(number_str) = file_name_str.strip_suffix(".yml") {
                    if let Ok(number) = number_str.parse::<usize>() {
                        objs.push(self.find_object(number)?)
                    }
                }

            } else {
                continue;
            }
        }

        Ok(objs)
    }
    
    pub fn update_object(&mut self, obj: &mut Object) -> Result<Object, ModuleError> {
        self.create_object(obj)
    }
    
    pub fn update_draft_object(&mut self, obj: &mut Object) -> Result<Object, ModuleError> {
        self.create_draft_object(obj)
    }

    pub fn delete_object(&mut self, id: usize) -> Result<Object, ModuleError> {
        let mut obj = self.find_object(id)?;
        obj.deleted_at = Some(Utc::now());
        Ok(self.update_object(&mut obj)?)
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
        
    pub fn update_draft_objects(path: &PathBuf, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
        todo!()
    }
    
    pub fn delete_draft_objects(path: &PathBuf, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
        todo!()
    }

    pub fn create_template(&self, template: Template) -> Result<Template, ModuleError> {
        mid::create_yml_file(&self.path, defs::OD_TEMPLATE_FILE_NAME, &template)?;
        Ok(self.read_template()?)
    }
    
    pub fn read_template(&self) -> Result<Template, ModuleError> {
        Ok(mid::read_yml_file::<Template,_>(&self.path, defs::OD_TEMPLATE_FILE_NAME)?)
    }
    
    pub fn update_template(&self, template: Template) -> Result<Template, ModuleError> {
        Ok(self.create_template(template)?)
    }

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

    fn get_next_available_id(&self) -> Result<usize, ModuleError> {
        let obj_next: usize = Module::next_file_number(&self.path.join(defs::OD_OBJS_FOLDER_NAME))?;
        let draft_next: usize = Module::next_file_number(&self.path.join(defs::OD_DRAFT_FOLDER_NAME))?;
        let max: usize = max(obj_next, draft_next);
        Ok(max)
    }

    fn next_file_number(dir: &PathBuf) -> Result<usize, ModuleError> {
        let mut max_number = 0;
    
        for entry in mid::read_folder(dir)? {
            let entry = entry;
            if entry.is_err() {
                return Err(ModuleError::UnknownError);
            } 
            let entry = entry.ok().unwrap();
            let file_name = entry.file_name();
            let file_name_str = file_name.to_str().unwrap_or("");
    
            if file_name_str.ends_with(".yml") {
                if let Some(number_str) = file_name_str.strip_suffix(".yml") {
                    if let Ok(number) = number_str.parse::<usize>() {
                        max_number = max(max_number, number);
                    }
                }
            }
        }
    
        Ok(max_number + 1)
    }

    fn save_object(&self, folder: &str, obj: &mut Object) -> Result<usize, ModuleError> {
        obj.assign_id(self.get_next_available_id()?);
        mid::create_yml_file(&self.path.join(folder), format!("{}.yml", obj.id()), &obj)?;
        Ok(obj.id())
    }

    fn open_object(path: &PathBuf, id: usize) -> Result<Object, ModuleError> {
        Ok(mid::read_yml_file::<Object, _>(path, format!("{id}.yml"))?)
    }

}
