use std::{cmp::max, collections::HashMap, path::PathBuf, str, vec};

use git2::Repository;
use chrono::Utc;
use serde::{Serialize, Deserialize};

use crate::core::{error::ModuleError, git, middleware as mid};
use super::{baseline::{Baseline, SemVer}, definitions as defs, links::Link, object::Object, template::Template};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModuleManifest {
	pub title: String,
	pub prefix: String,
	pub separator: String,
	pub description: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Module{
	pub path: PathBuf,
	pub manifest: ModuleManifest,
	pub template: Template,
	pub baselines: Vec<Baseline>,
	pub inbound_links: HashMap<usize, Vec<Link>>,
}

impl Module {
	pub fn create(repo: &Option<Repository>, path: &PathBuf, man: &ModuleManifest) -> Result<Module, ModuleError> {
		let repo: &Repository = Module::repo(&repo)?;
		let module_path: PathBuf = mid::create_folder(&path, &man.prefix)?;
		let baselines: Vec<Baseline> = Vec::new();
		let template: Template = Template::default();
		let inbound_links: HashMap<usize, Vec<Link>> = HashMap::new();
		
		mid::create_yml_file(&module_path, defs::OD_MODULE_MANIFEST_FILE_NAME, &man)?;
		mid::create_yml_file(&module_path, defs::OD_BASELINE_FILE_NAME, &baselines)?;
		mid::create_yml_file(&module_path, defs::OD_TEMPLATE_FILE_NAME, &template)?;
		mid::create_yml_file(&module_path, defs::OD_LINKS_FILE_NAME, &inbound_links)?;

		mid::create_file(
			&mid::create_folder(&module_path, defs::OD_OBJS_FOLDER_NAME)?,
			defs::OD_DUMMY_FILENAME
		)?;
	
		mid::create_file(
			&mid::create_folder(&module_path, defs::OD_DRAFT_FOLDER_NAME)?,
			defs::OD_DUMMY_FILENAME
		)?;
		
		mid::create_file(
			&mid::create_folder(&module_path, defs::OD_ASSETS_FOLDER_NAME)?,
			defs::OD_DUMMY_FILENAME
		)?;

		git::add_folder(&repo, &module_path.to_string_lossy())?;
		git::git_commit(&repo, &format!("Created module `{}`.", man.prefix))?;

		Ok(Module { 
			path: module_path, 
			manifest: man.clone(), 
			template,
			baselines,
			inbound_links,
		})
	}
	
	pub fn read(path: &PathBuf) -> Result<Module, ModuleError> {
		Module::check_for_module_folder(&path)?;
		let manifest: ModuleManifest = mid::read_yml_file(&path, defs::OD_MODULE_MANIFEST_FILE_NAME)?;
		let template: Template = mid::read_yml_file(&path, defs::OD_TEMPLATE_FILE_NAME)?;
		let baselines: Vec<Baseline> = mid::read_yml_file(&path, defs::OD_BASELINE_FILE_NAME)?;
		let inbound_links: HashMap<usize, Vec<Link>> = mid::read_yml_file(&path, defs::OD_LINKS_FILE_NAME)?;
		
		Ok(Module { 
			path: path.clone(), 
			manifest, 
			template, 
			baselines,
			inbound_links,
		})
	}
	
	pub fn update(repo: &Option<Repository>, path: &PathBuf, man: &ModuleManifest) -> Result<ModuleManifest, ModuleError> {
		Module::check_for_module_folder(&path)?;
		let repo: &Repository = Module::repo(&repo)?;
		let manifest_path: PathBuf = mid::update_yml_file(&path, defs::OD_MODULE_MANIFEST_FILE_NAME, &man)?;
		git::add_file(&repo, &manifest_path.to_string_lossy())?;
		git::git_commit(&repo, &format!("Updated the manifest of module `{}`.", man.prefix))?;
		Ok(mid::read_yml_file::<ModuleManifest, _>(&path, defs::OD_MODULE_MANIFEST_FILE_NAME)?)
	}
	
	pub fn delete(repo: &Option<Repository>, path: &PathBuf) -> Result<(), ModuleError> {
		let repo: &Repository = Module::repo(&repo)?;
		let repo_path: PathBuf = PathBuf::from(repo.path());
		let module_location: std::borrow::Cow<'_, str> = path.strip_prefix(repo_path).unwrap_or(path).to_string_lossy();
		Module::check_for_module_folder(&path)?;
		mid::delete_folder(&path)?;
		git::add_folder(&repo, &path.to_string_lossy())?;
		git::git_commit(&repo, &format!("Deleted module located at `{}`.", &module_location))?;
		Ok(())
	}

	pub fn read_object(&self, id: usize) -> Result<Object, ModuleError> {
		Ok(Module::open_object(&self.path.join(defs::OD_OBJS_FOLDER_NAME), id)?)
	}

	pub fn read_draft_object(&self, id: usize) -> Result<Object, ModuleError> {
		Ok(Module::open_object(&self.path.join(defs::OD_DRAFT_FOLDER_NAME), id)?)
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
	
	fn prepare_object(&mut self, repo: &Option<Repository>, obj: &mut Object) -> Result<String, ModuleError> {
		let id: usize = self.save_object(defs::OD_DRAFT_FOLDER_NAME, obj)?;
		let filename: String = format!("{id}.yml");

		if let Some(outbound_links) = &obj.outbound_links {
			let mut valid_links: Vec<Link> = Vec::new();
			let repo_path: PathBuf = self.get_repository_path().unwrap_or_default();
			let rel_path: PathBuf = Module::subtract_paths(&repo_path, &self.path).unwrap_or(self.path.clone());

			let inbound_link: Link = Link { 
				path: rel_path,
				object: id.clone(),
				module: self.manifest.prefix.clone(),
			};
			for outbound_link in outbound_links {
				let module_path: PathBuf = Module::add_paths(&repo_path, &outbound_link.path);
				match Module::read(&module_path) {
					Err(_) => continue,
					Ok(module) => {
						module.create_inbound_link(repo, &inbound_link, &outbound_link.object)?;
						valid_links.push(outbound_link.clone());
					},
				}
			}

			if valid_links.is_empty() {
				obj.outbound_links = None;
			}
		}

		self.save_object(defs::OD_DRAFT_FOLDER_NAME, obj)?;

		let origin = &self.path.join(defs::OD_DRAFT_FOLDER_NAME);
		let destination = &self.path.join(defs::OD_OBJS_FOLDER_NAME);
		if let Err(_) = mid::move_file(origin, destination, &filename) {
			mid::create_folder(&self.path, defs::OD_OBJS_FOLDER_NAME)?;
			mid::move_file(origin, destination, &filename)?;
		}
		
		Ok(destination.join(filename).to_string_lossy().into())
	}

	pub fn create_object(&mut self, repo: &Option<Repository>, obj: &mut Object) -> Result<Object, ModuleError> {
		let obj_path = self.prepare_object(&repo, obj)?;
		let repo: &Repository = Module::repo(&repo)?;
		git::add_file(&repo, &obj_path)?;
		git::git_commit(&repo, &format!("Created object `{}:{}`.", self.manifest.prefix, obj.id()))?;
		Ok(self.read_object(obj.id())?)
	}
	
	pub fn create_draft_object(&mut self, obj: &mut Object) -> Result<Object, ModuleError> {
		let id: usize = self.save_object(defs::OD_DRAFT_FOLDER_NAME, obj)?;
		Ok(self.read_draft_object(id)?)
	}

	pub fn create_objects(&mut self, repo: &Option<Repository>, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
		let mut res: Vec<Object> = Vec::new();
		for obj in objs {
			let mut obj = obj.clone();
			res.push(self.create_object(repo, &mut obj)?);
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
	
	pub fn read_objects(&self) -> Result<Vec<Object>, ModuleError> {
		let mut objs: Vec<Object> = Vec::new();
		let entries = mid::read_folder(&self.path.join(defs::OD_OBJS_FOLDER_NAME));

		if entries.is_err() {
			mid::create_folder(&self.path, defs::OD_OBJS_FOLDER_NAME)?;
			return Ok(objs);
		}

		for entry in entries? {
			if let Ok(entry) = entry {
				let file_name = entry.file_name();
				let file_name_str = file_name.to_str().unwrap_or("");
		
				if !file_name_str.ends_with(".yml") {
					continue;
				}
	
				if let Some(number_str) = file_name_str.strip_suffix(".yml") {
					if let Ok(number) = number_str.parse::<usize>() {
						objs.push(self.read_object(number)?)
					}
				}
			}
		}

		Ok(Module::sort_by_level(objs))
	}

	pub fn read_draft_objects(&mut self) -> Result<Vec<Object>, ModuleError> {
		let mut objs: Vec<Object> = Vec::new();
		let entries = mid::read_folder(&self.path.join(defs::OD_DRAFT_FOLDER_NAME));

		if entries.is_err() {
			mid::create_folder(&self.path, defs::OD_DRAFT_FOLDER_NAME)?;
			return Ok(objs);
		}

		for entry in mid::read_folder(&self.path.join(defs::OD_DRAFT_FOLDER_NAME))? {
			if let Ok(entry) = entry {
				let file_name = entry.file_name();
				let file_name_str = file_name.to_str().unwrap_or("");
		
				if !file_name_str.ends_with(".yml") {
					continue;
				}
	
				if let Some(number_str) = file_name_str.strip_suffix(".yml") {
					if let Ok(number) = number_str.parse::<usize>() {
						objs.push(self.read_draft_object(number)?);
					}
				}
			}
		}
		
		Ok(Module::sort_by_level(objs))
	}
	
	pub fn update_object(&mut self, repo: &Option<Repository>, obj: &mut Object) -> Result<Object, ModuleError> {
		let obj_path = self.prepare_object(&repo, obj)?;
		let repo = Module::repo(&repo)?;
		git::add_file(&repo, &obj_path)?;
		git::git_commit(&repo, &format!("Updated object `{}:{}`.", self.manifest.prefix, obj.id()))?;
		Ok(self.read_object(obj.id())?)
	}
	
	pub fn update_draft_object(&mut self, obj: &mut Object) -> Result<Object, ModuleError> {
		self.create_draft_object(obj)
	}

	pub fn delete_object(&mut self, repo: &Option<Repository>, id: usize) -> Result<Object, ModuleError> {
		let mut obj = self.find_object(id)?;
		obj.deleted_at = Some(Utc::now());
		let obj_path = self.prepare_object(&repo, &mut obj)?;
		let repo_path = self.get_repository_path().ok_or(ModuleError::NoRepositoryInitialized)?;
		if let Some(outbound_links) = &obj.outbound_links {
			let inbound_link: Link = Link { 
				path: self.path.strip_prefix(&repo_path).unwrap_or(&self.path).into(),
				object: obj.id(),
				module: self.manifest.prefix.clone(),
			};
			println!("{:?}", inbound_link);
			for outbound_link in outbound_links {
				let dest_mod: Module = Module::read(&repo_path.join(&outbound_link.path))?;
				dest_mod.delete_inbound_link(&repo, &inbound_link)?;
			}
		}
		let repo = Module::repo(&repo)?;
		git::add_file(&repo, &obj_path)?;
		git::git_commit(&repo, &format!("Deleted object `{}:{}`.", self.manifest.prefix, obj.id()))?;
		Ok(self.read_object(obj.id())?)
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

	pub fn create_template(&self, repo: &Option<Repository>, template: Template) -> Result<Template, ModuleError> {
		let repo = Module::repo(&repo)?;
		let template_path = mid::create_yml_file(&self.path, defs::OD_TEMPLATE_FILE_NAME, &template)?;
		git::add_file(&repo, &template_path.to_string_lossy())?;
		git::git_commit(&repo, &format!("Created Module Template file for module {}.", self.manifest.prefix))?;
		Ok(self.read_template()?)
	}
	
	pub fn read_template(&self) -> Result<Template, ModuleError> {
		Ok(mid::read_yml_file::<Template,_>(&self.path, defs::OD_TEMPLATE_FILE_NAME)?)
	}
	
	pub fn update_template(&self, repo: &Option<Repository>, template: Template) -> Result<Template, ModuleError> {
		let repo = Module::repo(&repo)?;
		let template_path = mid::create_yml_file(&self.path, defs::OD_TEMPLATE_FILE_NAME, &template)?;
		git::add_file(&repo, &template_path.to_string_lossy())?;
		git::git_commit(&repo, &format!("Updated Module Template file for module {}.", self.manifest.prefix))?;
		Ok(self.read_template()?)
	}

	pub fn create_baseline(&self, repo: &Option<Repository>, semver: &str, desc: &str) -> Result<Vec<Baseline>, ModuleError> {
		let version: String = format!("{}/{}", self.manifest.prefix.to_lowercase(), semver);
		let description: String = desc.to_owned();
		let repo: &Repository = Module::repo(&repo)?;
		let hash: String = git::create_tag(&repo, &version, &description)?;
		let path = self.path.clone();
		let baseline: Baseline = Baseline { 
			version: SemVer::from(&version), 
			hash: Some(hash), 
			description, 
		};

		let mut baselines: Vec<Baseline> = mid::read_yml_file(&path, defs::OD_BASELINE_FILE_NAME)?;
		baselines.push(baseline);
		let baselines_path = mid::update_yml_file(&path, defs::OD_BASELINE_FILE_NAME, &baselines)?;
		git::add_file(&repo, &baselines_path.to_string_lossy())?;
		git::git_commit(&repo, &format!("Baselined module `{}` at version `{}` - `{}`.", self.manifest.prefix, version, desc))?;
		Ok(baselines)
	}
	
	pub fn read_baselines(path: &PathBuf) -> Result<Vec<Baseline>, ModuleError> {
		let mut baselines: Vec<Baseline> = mid::read_yml_file(&path, defs::OD_BASELINE_FILE_NAME)?;
		Ok(baselines)
	}

	pub fn read_from_baseline(&mut self, repo: &Option<Repository>, path: &PathBuf, baseline: Baseline) -> Result<Vec<Object>, ModuleError> {
		let repo: &Repository = Module::repo(repo)?;
		if let Some(spec) = baseline.hash {
			let tree = repo.revparse_single(&spec)?.peel_to_commit()?.tree()?;
			let entry = tree.get_path(&path)?;
			let blob = repo.find_blob(entry.id())?;
			let content = unsafe { str::from_boxed_utf8_unchecked(blob.content().into()) };
			println!("Baseline Content: {}, ", content);
		}
		return self.read_objects();
	}

	pub fn create_inbound_link(&self, repo: &Option<Repository>, link: &Link, id: &usize) -> Result<(), ModuleError> {
		let repo: &Repository = Module::repo(repo)?;
		let mut links: HashMap<usize, Vec<Link>> = mid::read_yml_file(&self.path, defs::OD_LINKS_FILE_NAME)?;
		if Module::add_unique_link(&mut links, &id, &link) {
			git::add_file(&repo, &mid::update_yml_file(&self.path, defs::OD_LINKS_FILE_NAME, &links)?.to_string_lossy())?;
			git::git_commit(&repo, &format!("Create link of `{}:{}` to `{}:{}`.", self.manifest.prefix, id, link.module, link.object))?;
		}
		Ok(())
	}

	pub fn read_inbound_links(&self) -> Result<HashMap<usize, Vec<Link>>, ModuleError> {
		Ok(mid::read_yml_file(&self.path, defs::OD_LINKS_FILE_NAME)?)
	}

	pub fn delete_inbound_link(&self, repo: &Option<Repository>, link: &Link) -> Result<HashMap<usize, Vec<Link>>, ModuleError> {
		let mut links: HashMap<usize, Vec<Link>> = mid::read_yml_file(&self.path, defs::OD_LINKS_FILE_NAME)?;
		let mut empty_keys: Vec<usize> = Vec::new();
		let repo: &Repository = Module::repo(repo)?;

		for (id, links) in links.iter_mut() {
			links.retain(|x| x != link);
			if links.is_empty() {
				empty_keys.push(*id);
			}
		}

		for id in &empty_keys {
			links.remove(&id);
		}

		git::add_file(&repo, &mid::update_yml_file(&self.path, defs::OD_LINKS_FILE_NAME, &links)?.to_string_lossy())?;
		git::git_commit(&repo, &format!("Deleted link of `{}:{:?}` to `{}:{}`.", self.manifest.prefix, &empty_keys, link.module, link.object))?;

		self.read_inbound_links()
	}

	fn get_repository_path(&self) -> Option<PathBuf> {
		let mut current_path = self.path.clone();

		loop {
			let repository_yml_path = current_path.join("repository.yml");

			if repository_yml_path.exists() {
				return Some(current_path);
			}

			if !current_path.pop() {
				break;
			}
		}

		None
	}

	fn subtract_paths(base: &PathBuf, absolute: &PathBuf) -> Option<PathBuf> {
		absolute.strip_prefix(base).ok().map(|p| p.to_path_buf())
	}

	fn add_paths(base: &PathBuf, relative: &PathBuf) -> PathBuf {
		return base.clone().join(relative);
	}

	fn add_unique_link(map: &mut HashMap<usize, Vec<Link>>, id: &usize, link: &Link) -> bool {
		let entry = map.entry(id.clone()).or_insert_with(Vec::new);
		if entry.iter().any(|l| *l == *link) {
			false
		} else {
			entry.push(link.clone());
			true		
		}
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
		let entries = mid::read_folder(dir);

		if entries.is_err() {
			return Ok(max_number);
		}

		for entry in entries? {
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
		if let Err(_) = mid::create_yml_file(&self.path.join(folder), format!("{}.yml", obj.id()), &obj) {
			mid::create_folder(&self.path, folder)?;
			mid::create_yml_file(&self.path.join(folder), format!("{}.yml", obj.id()), &obj)?;
		}
		Ok(obj.id())
	}

	fn open_object(path: &PathBuf, id: usize) -> Result<Object, ModuleError> {
		Ok(mid::read_yml_file::<Object, _>(path, format!("{id}.yml"))?)
	}

	fn sort_by_level(mut objects: Vec<Object>) -> Vec<Object> {

		fn compare_levels(a: &str, b: &str) -> i32 {
			fn parse_level(level: &str) -> Vec<Result<i32, &str>> {
				level
					.split(|c| c == '.' || c == '-')
					.filter(|part| !part.is_empty())
					.map(|part| {
						let part = part.trim_end_matches('0');
						if part.is_empty() {
							Ok(0)
						} else {
							part.parse::<i32>().map_err(|_| part)
						}
					})
					.collect()
			}
		
			let a_parts = parse_level(a);
			let b_parts = parse_level(b);
		
			let len = std::cmp::max(a_parts.len(), b_parts.len());
		
			for i in 0..len {
				let a_part = a_parts.get(i);
				let b_part = b_parts.get(i);

				if a_part.is_none() {
					return -1;
				}

				if b_part.is_none() {
					return 1;
				}

				match (a_part.unwrap(), b_part.unwrap()) {
					(Ok(a_num), Ok(b_num)) => {
						if a_num != b_num {
							return a_num - b_num;
						}
					}
					(Err(a_str), Err(b_str)) => {
						let cmp = a_str.cmp(b_str);
						if cmp != std::cmp::Ordering::Equal {
							return cmp as i32;
						}
					}
					(Ok(_), Err(_)) => return -1,
					(Err(_), Ok(_)) => return 1,
				}
			}

			0
		}
		
		objects.sort_by(|a, b| compare_levels(&a.level, &b.level).cmp(&0));

		objects
	}

	fn repo(repo: &Option<Repository>) -> Result<&Repository, ModuleError> {
		repo.as_ref().ok_or(ModuleError::NoRepositoryInitialized)
	}
}
