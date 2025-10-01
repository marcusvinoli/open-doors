use core::str;
use std::{cmp::max, collections::HashMap, path::{Path, PathBuf}};

use chrono::Utc;
use serde::{Serialize, Deserialize};
use git2::{Repository, Tree, TreeEntry, ObjectType};

use crate::core::{error::ModuleError, utils::SemVer, User, git, middleware as mid};
use super::{definitions as defs, Baseline, BaselineStatus, Link, Links, Object, ObjectStatus, Template, View};

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
	pub views: Vec<View>,
	pub baselines: Vec<Baseline>,
	pub links: Links
}

impl Module {
	pub fn create(repo: &Option<Repository>, path: &PathBuf, man: &ModuleManifest) -> Result<Module, ModuleError> {
		let repo: &Repository = Module::repo(&repo)?;
		let module_path: PathBuf = mid::create_folder(&path, &man.prefix)?;
		let baselines: Vec<Baseline> = Vec::new();
		let template: Template = Template::default();
		let views: Vec<View> = Vec::new();
		let links: Links = Links::default();
		
		mid::create_yml_file(&module_path, defs::OD_MODULE_MANIFEST_FILE_NAME, &man)?;
		mid::create_yml_file(&module_path, defs::OD_BASELINE_FILE_NAME, &baselines)?;
		mid::create_yml_file(&module_path, defs::OD_TEMPLATE_FILE_NAME, &template)?;
		mid::create_yml_file(&module_path, defs::OD_VIEWS_FILE_NAME, &views)?;
		mid::create_yml_file(&module_path, defs::OD_LINKS_FILE_NAME, &links)?;
		
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
			views,
			baselines,
			links,
		})
	}
	
	pub fn read(path: &PathBuf) -> Result<Module, ModuleError> {
		Module::check_for_module_folder(&path)?;
		let manifest: ModuleManifest = mid::read_yml_file(&path, defs::OD_MODULE_MANIFEST_FILE_NAME)?;
		let template: Template = mid::read_yml_file(&path, defs::OD_TEMPLATE_FILE_NAME)?;
		let baselines: Vec<Baseline> = mid::read_yml_file(&path, defs::OD_BASELINE_FILE_NAME)?;
		let views: Vec<View> = mid::read_yml_file(&path, defs::OD_VIEWS_FILE_NAME)?;
		let links: Links = mid::read_yml_file(&path, defs::OD_LINKS_FILE_NAME)?;
		
		Ok(Module { 
			path: path.clone(), 
			manifest,
			template,
			views, 
			baselines,
			links,
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
		let module = Module::read(path)?;
		for baseline in &module.baselines {
			if baseline.hash.is_none() {
				continue;
			}
			let _ = repo.tag_delete(&Module::tag_format(&module, &baseline.version.to_string()));
		}
		mid::delete_folder(&path)?;
		git::add_folder(&repo, &path.to_string_lossy())?;
		git::git_commit(&repo, &format!("Deleted module located at `{}`.", &module_location))?;
		Ok(())
	}

	pub fn create_object(&mut self, repo: &Option<Repository>, obj: &mut Object) -> Result<Object, ModuleError> {
		let obj_path: String = self.prepare_object(obj)?;
		let repo: &Repository = Module::repo(&repo)?;
		git::add_file(&repo, &obj_path)?;
		git::git_commit(&repo, &format!("Created object `{}:{}`.", self.manifest.prefix, obj.id()))?;
		let mut obj: Object = self.read_object(obj.id())?;
		obj.set_status(ObjectStatus::Updated);
		Ok(obj)
	}
	
	pub fn create_draft_object(&mut self, obj: &mut Object) -> Result<Object, ModuleError> {
		let id: usize = self.save_object(defs::OD_DRAFT_FOLDER_NAME, obj)?;
		let mut obj: Object = self.read_draft_object(id)?;
		obj.set_status(ObjectStatus::Draft);
		Ok(obj)
	}

	pub fn create_draft_objects(&mut self, objs: &Vec<Object>) -> Result<Vec<Object>, ModuleError> {
		let mut res: Vec<Object> = Vec::new();
		for obj in objs {
			let mut obj = obj.clone();
			res.push(self.create_draft_object(&mut obj)?);
		}
		Ok(res)
	}

	pub fn read_object(&self, id: usize) -> Result<Object, ModuleError> {
		let mut obj = Module::open_object(&self.path.join(defs::OD_OBJS_FOLDER_NAME), id)?;
		obj.add_metadata(ObjectStatus::Updated, &self.links);
		if obj.deleted_at.is_some() {
			obj.set_status(ObjectStatus::Deleted);
		} else if let Some(lts) = self.get_latest_baseline() {
			if obj.updated_at < lts.created_at {
				obj.set_status(ObjectStatus::Baselined);
			}
		}
		Ok(obj)
	}

	pub fn read_draft_object(&self, id: usize) -> Result<Object, ModuleError> {
		let mut obj: Object = Module::open_object(&self.path.join(defs::OD_DRAFT_FOLDER_NAME), id)?;
		obj.add_metadata(ObjectStatus::Draft, &self.links);
		Ok(obj)
	}

	pub fn read_objects(&self) -> Result<Vec<Object>, ModuleError> {
		let comm_objs: Vec<Object> = self.get_commited_objects()?;
		let draft_objs: Vec<Object> = self.get_draft_objects()?;
		let mut map: HashMap<usize, Object> = comm_objs
			.into_iter()
			.map(|obj| (obj.id(), obj))
			.collect();

		for obj in draft_objs {
			map.insert(obj.id(), obj); 
		}
		Ok(Module::sort_by_level(map.into_values().collect()))
	}

	pub fn update_object(&mut self, repo: &Option<Repository>, obj: &mut Object) -> Result<Object, ModuleError> {
		let obj_path = self.prepare_object(obj)?;
		let repo = Module::repo(&repo)?;
		git::add_file(&repo, &obj_path)?;
		git::git_commit(&repo, &format!("Updated object `{}:{}`.", self.manifest.prefix, obj.id()))?;
		let mut obj: Object = self.read_object(obj.id())?;
		obj.add_metadata(ObjectStatus::Updated, &self.links);
		Ok(obj)
	}
	
	pub fn update_draft_object(&mut self, obj: &mut Object) -> Result<Object, ModuleError> {
		self.create_draft_object(obj)
	}

	pub fn delete_object(&mut self, repo: &Option<Repository>, id: usize) -> Result<Object, ModuleError> {
		let mut obj = self.find_object(id)?;
		let now = Utc::now();
		obj.deleted_at = Some(now.clone());
		obj.updated_at = now.into();
		let obj_path = self.prepare_object(&mut obj)?;
		let repo = Module::repo(&repo)?;
		git::add_file(&repo, &obj_path)?;
		git::git_commit(&repo, &format!("Deleted object `{}:{}`.", self.manifest.prefix, obj.id()))?;
		let mut obj: Object = self.read_object(obj.id())?;
		obj.add_metadata(ObjectStatus::Deleted, &self.links);
		Ok(obj)
	}

	pub fn restore_object(&mut self, repo: &Option<Repository>, id: usize) -> Result<Object, ModuleError> {
		let mut obj = self.find_object(id)?;
		obj.deleted_at = None;
		let obj_path = self.prepare_object(&mut obj)?;
		let repo = Module::repo(&repo)?;
		git::add_file(&repo, &obj_path)?;
		git::git_commit(&repo, &format!("Restored object `{}:{}`.", self.manifest.prefix, obj.id()))?;
		let mut obj: Object = self.read_object(obj.id())?;
		obj.add_metadata(ObjectStatus::Updated, &self.links);
		Ok(obj)
	}

	fn find_object(&self, id: usize) -> Result<Object, ModuleError> {
		match Module::open_object(&self.path.join(defs::OD_OBJS_FOLDER_NAME), id) {
			Ok(obj) => {
				return Ok(obj)
			},
			Err(_) => {
				return Module::open_object(&self.path.join(defs::OD_DRAFT_FOLDER_NAME), id)
			}
		};
	}

	fn get_commited_objects(&self) -> Result<Vec<Object>, ModuleError> {
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
						objs.push(self.read_object(number)?);
					}
				}
			}
		}

		Ok(objs)
	}

	fn get_draft_objects(&self) -> Result<Vec<Object>, ModuleError> {
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
		
		Ok(objs)
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

	pub fn read_views(&self) -> Result<Vec<View>, ModuleError> {
		Ok(mid::read_yml_file::<Vec<View>,_>(&self.path, defs::OD_VIEWS_FILE_NAME)?)
	}

	pub fn update_views(&self, repo: &Option<Repository>, view: Vec<View>) -> Result<Vec<View>, ModuleError> {
		let repo = Module::repo(&repo)?;
		let view_path = mid::create_yml_file(&self.path, defs::OD_VIEWS_FILE_NAME, &view)?;
		git::add_file(&repo, &view_path.to_string_lossy())?;
		git::git_commit(&repo, &format!("Updated Views for module {}.", self.manifest.prefix))?;
		Ok(self.read_views()?)
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

	pub fn create_baseline(&mut self, repo: &Option<Repository>, semver: &str, desc: &str) -> Result<Vec<Baseline>, ModuleError> {
		let version: String = self.tag_from_semver(semver);
		let description: String = desc.to_owned();
		let repo: &Repository = Module::repo(&repo)?;
		let user: User = User::from_repository(&repo)?;
		let hash: String = git::create_tag(&repo, &version, &description)?;
		let path = self.path.clone();
		let baseline: Baseline = Baseline { 
			version: SemVer::from(&semver),
			created_at: Utc::now(),
			created_by: user.to_string(),
			description,
			hash: Some(hash),
			deleted_at: None,
			deleted_by: None,
			status: BaselineStatus::Latest,
		};
		let mut baselines: Vec<Baseline> = mid::read_yml_file(&path, defs::OD_BASELINE_FILE_NAME)?;
		baselines.iter_mut().for_each(|bl| {
			if bl.status == BaselineStatus::Latest {
				bl.status = BaselineStatus::Historical
			}
		});
		baselines.push(baseline);
		let baselines_path = mid::update_yml_file(&path, defs::OD_BASELINE_FILE_NAME, &baselines)?;
		self.baselines = baselines;
		git::add_file(&repo, &baselines_path.to_string_lossy())?;
		git::git_commit(&repo, &format!("Module `{}` baselined at version `{}` - `{}`.", self.manifest.prefix, version, desc))?;
		Ok(self.baselines.to_owned())
	}
	
	pub fn read_baselines(&mut self) -> Result<Vec<Baseline>, ModuleError> {
		self.baselines = mid::read_yml_file(&self.path, defs::OD_BASELINE_FILE_NAME)?;
		Ok(self.baselines.to_owned())
	}

	pub fn delete_baselines(&mut self, repo: &Option<Repository>, version: &SemVer) -> Result<Vec<Baseline>, ModuleError> {
		let repo: &Repository = Module::repo(&repo)?;
		let user: User = User::from_repository(&repo)?;
		self.baselines.iter_mut().for_each(|bl| {
			if bl.version == *version {
				bl.deleted_at = Some(Utc::now());
				bl.deleted_by = Some(user.to_string());
				bl.status = BaselineStatus::Deleted;
			}
		});
		let baselines_path = mid::update_yml_file(&self.path, defs::OD_BASELINE_FILE_NAME, &self.baselines)?;
		git::add_file(&repo, &baselines_path.to_string_lossy())?;
		git::git_commit(&repo, &format!("Baseline deleted `{}` at version `{}`.", self.manifest.prefix, version.to_string()))?;
		self.read_baselines()
	}

	fn baseline_hash(&self, version: &str) -> Result<Baseline, ModuleError> {
		for baseline in &self.baselines {
			if &baseline.version.to_string() == version {
				return Ok(baseline.clone());
			}
		}
		Err(ModuleError::BaselineNotFound(version.into()))
	}

	fn get_latest_baseline(&self) -> Option<&Baseline> {
		self.baselines.iter().find(|bl| bl.status == BaselineStatus::Latest)
	}

	fn object_relative_folder(&self) -> String {
		let folder_path: &PathBuf = &self.path.join(defs::OD_OBJS_FOLDER_NAME);
		folder_path.to_string_lossy().into()
	}

	fn module_relative_path(&self, repo: &Option<Repository>) -> Result<String, ModuleError> {
		let module_path: &PathBuf = &self.path.clone();
		let repo: &Repository = Module::repo(repo)?;
		let mut repo_path: PathBuf = PathBuf::from(repo.path());
		repo_path.pop();
		let rel_path: PathBuf = PathBuf::from(module_path);
		let path = if let Ok(relative) = rel_path.strip_prefix(&repo_path) {
			relative
		} else {
			rel_path.as_path()
		};
		Ok(path.to_string_lossy().into())
	}

	fn object_relative_path(&self, repo: &Option<Repository>, id: &usize) -> Result<String, ModuleError> {
		let file_name: String = format!("{}.yml", id);
		let object_path: &PathBuf = &self.path.join(defs::OD_OBJS_FOLDER_NAME).join(file_name);
		let repo: &Repository = Module::repo(repo)?;
		let mut repo_path: PathBuf = PathBuf::from(repo.path());
		repo_path.pop();
		let rel_path: PathBuf = PathBuf::from(object_path);
		let path = if let Ok(relative) = rel_path.strip_prefix(&repo_path) {
			relative
		} else {
			rel_path.as_path()
		};
		Ok(path.to_string_lossy().into())
	}

	fn tree_entry(&self, repo: &Repository, hash: &Option<String>, path: &str) -> Result<TreeEntry, ModuleError> {
		let hash = hash.as_ref().ok_or(ModuleError::BaselineNotCommited)?;
		let tree = repo.revparse_single(&hash)?.peel_to_commit()?.tree()?;
		Ok(tree.get_path(&PathBuf::from(&path))?)
	}

	fn find_subtree<'a>(&self, repo: &'a Repository, mut tree: Tree<'a>, path: &'a str) -> Result<Tree<'a>, ModuleError> {
		let path = Path::new(path);
		for component in path.components() {
			let component_str = component.as_os_str().to_str().ok_or(ModuleError::GenericError("Error handling module path.".into()))?;
			let mut found = None;
			for entry in tree.iter() {
				if entry.name() == Some(component_str) && entry.kind() == Some(ObjectType::Tree) {
					found = Some(entry.id());
					break;
				}
			}
			let subtree_id = found.ok_or(ModuleError::GenericError(format!("Diretório '{}' não encontrado", component_str)))?;
			tree = repo.find_tree(subtree_id)?;
		}
		Ok(tree)
	}

	pub fn read_file_from_tag_at_path(&self, repo: &Repository, version: &str, file_path: &str) -> Result<String, ModuleError> {
		// Busca a tag e resolve o commit correspondente
		let tag: String = self.tag_from_semver(version);
		let reference = repo.revparse_single(&format!("refs/tags/{}", tag))?;
		let commit = reference.peel_to_commit()?;

		// Obtém a árvore de arquivos desse commit
		let tree = commit.tree()?;

		// Caminho dentro do repositório (ex: "src/main.rs")
		let entry = tree.get_path(Path::new(file_path))?;
		let object = entry.to_object(&repo)?;

		// Extrai o blob (conteúdo do arquivo)
		if let Some(blob) = object.as_blob() {
			let content = std::str::from_utf8(blob.content())
				.map_err(|_| ModuleError::GenericError("Arquivo não é UTF-8 válido".into()))?;
			Ok(content.to_string())
		} else {
			Err(ModuleError::GenericError("Objeto não é um blob".into()))
		}
	}

	fn _read_file_from_tag_at_path(&self, repo: &Repository, hash: &Option<String>, folder_path: &str, file_name: &str) -> Result<String, ModuleError> {
		let hash = hash.as_ref().ok_or(ModuleError::BaselineNotCommited)?;
		let tag_obj = repo.revparse_single(&hash)?;
		let tag_commit = tag_obj.peel_to_commit()?;
		let root_tree = tag_commit.tree()?;
		let subtree = self.find_subtree(repo, root_tree, folder_path)?;
		let mut results = String::new();
		for entry in subtree.iter() {
			if entry.kind() == Some(ObjectType::Blob) {
				if let Some(name) = entry.name() {
					if name == file_name {
						let blob = repo.find_blob(entry.id())?;
						let content = str::from_utf8(blob.content())?;
						results = content.to_string();
						break;
					}
				}
			}
		}
		Ok(results)
	}
	
	pub fn read_files_from_tag_at_path(&self, repo: &Repository, hash: &Option<String>, folder_path: &str) -> Result<Vec<String>, ModuleError> {
		let hash = hash.as_ref().ok_or(ModuleError::BaselineNotCommited)?;
		let tag_obj = repo.revparse_single(&hash)?;
		let tag_commit = tag_obj.peel_to_commit()?;
		let root_tree = tag_commit.tree()?;
		let subtree = self.find_subtree(repo, root_tree, folder_path)?;
		let mut results = Vec::new();
		for entry in subtree.iter() {
			if entry.kind() == Some(ObjectType::Blob) {
				if let Some(name) = entry.name() {
					if name.ends_with(".yml") {
						let blob = repo.find_blob(entry.id())?;
						let content = str::from_utf8(blob.content())?;
						results.push(content.to_string());
					}
				}
			}
		}
		Ok(results)
	}

	pub fn read_links_from_baseline(&self, repo: &Option<Repository>, version: &str) -> Result<Links, ModuleError> {
		let file_path: PathBuf = PathBuf::from(self.module_relative_path(&repo)?).join(defs::OD_LINKS_FILE_NAME);
		let repo: &Repository = Module::repo(repo)?;
		let links: String = self.read_file_from_tag_at_path(repo, version, &file_path.to_string_lossy())?;
		let links: Links = serde_yaml::from_str(&links)?;
		Ok(links)
	}

	pub fn read_object_from_baseline(&self, repo: &Option<Repository>, id: &usize, version: &str) -> Result<Object, ModuleError> {
		let baseline: Baseline = self.baseline_hash(version)?;
		let path: String = self.object_relative_path(&repo, &id)?;
		let links: Links = self.read_links_from_baseline(&repo, version)?;
		let repo: &Repository = Module::repo(repo)?;
		let entry = self.tree_entry(&repo, &baseline.hash, &path)?;
		let blob = repo.find_blob(entry.id())?;
		let content = str::from_utf8(blob.content().into())?;
		let mut obj: Object = serde_yaml::from_str::<Object>(&content)?.to_owned();
		obj.add_metadata( ObjectStatus::Baselined, &links);
		if obj.deleted_at.is_some() {
			obj.set_status( ObjectStatus::Deleted);
		}
		Ok(serde_yaml::from_str(&content)?)
	}

	pub fn read_objects_from_baseline(&self, repo: &Option<Repository>, version: &str) -> Result<Vec<Object>, ModuleError> {
		let folder_path = PathBuf::from(self.module_relative_path(&repo)?).join(defs::OD_OBJS_FOLDER_NAME);
		let links: Links = self.read_links_from_baseline(&repo, &version)?;
		let baseline: Baseline = self.baseline_hash(version)?;
		let repo: &Repository = Module::repo(repo)?;
		let files: Vec<String> = self.read_files_from_tag_at_path(&repo, &baseline.hash, &folder_path.to_string_lossy())?;
		let mut objs: Vec<Object> = Vec::new();
		for file in files {
			let mut obj: Object = serde_yaml::from_str::<Object>(&file)?.to_owned();
			obj.add_metadata( ObjectStatus::Baselined, &links);
			if obj.deleted_at.is_some() {
				obj.set_status( ObjectStatus::Deleted);
			}
			objs.push(obj);
		}
		Ok(Module::sort_by_level(objs))
	}

	pub fn read_module_from_baseline(&self, repo: &Option<Repository>, semver: &str) -> Result<Module, ModuleError> {
		// TODO: This function should not use check-out to read the module. The convenient (and correct) procedure is to parse files from git database.
		let repo: &Repository = Module::repo(repo)?;
		let tag: String = self.tag_from_semver(semver);
		let original_ref = repo.head()?.target().ok_or(ModuleError::BaselineNotCommited)?;
		let tag_object = repo.revparse_single(&tag)?;
		repo.checkout_tree(&tag_object, None)?;
    	repo.set_head_detached(tag_object.id())?;
		let baselined_module = Module::read(&self.path)?;
		let original_commit = repo.find_commit(original_ref)?;
		repo.checkout_tree(&original_commit.as_object(), None)?;
		repo.set_head_detached(original_commit.id())?;
		Ok(baselined_module)
	}

	pub fn create_link(&mut self, repo: &Option<Repository>, origin: &Link, destination: &Link) -> Result<Links, ModuleError> {
		let repo_path: PathBuf = self.get_repository_path().unwrap_or_default();
		let dest_module_path: PathBuf = Module::add_paths(&repo_path, &PathBuf::from(&destination.path));
		let dest_module: Module = Module::read(&dest_module_path)?;
		dest_module.create_inbound_link(&repo, origin, destination)?;
		self.create_outbound_links(&repo, origin, destination)?;
		self.read_links()
	}

	fn create_inbound_link(&self, repo: &Option<Repository>, origin: &Link, destination: &Link) -> Result<(), ModuleError> {
		let repo: &Repository = Module::repo(repo)?;
		let mut links: Links = mid::read_yml_file(&self.path, defs::OD_LINKS_FILE_NAME)?;
		if Module::add_unique_link(&mut links.inbound_links, &destination.object, &origin) {
			git::add_file(&repo, &mid::update_yml_file(&self.path, defs::OD_LINKS_FILE_NAME, &links)?.to_string_lossy())?;
			git::git_commit(&repo, &format!("Create link of `{}:{}` to `{}:{}`.", destination.module(), destination.object, self.manifest.prefix, origin.object))?;
		}
		Ok(())
	}
	
	fn create_outbound_links(&self, repo: &Option<Repository>, origin: &Link, destination: &Link) -> Result<(), ModuleError> {
		let repo: &Repository = Module::repo(repo)?;
		let mut links: Links = mid::read_yml_file(&self.path, defs::OD_LINKS_FILE_NAME)?;
		if Module::add_unique_link(&mut links.outbound_links, &origin.object, &destination) {
			git::add_file(&repo, &mid::update_yml_file(&self.path, defs::OD_LINKS_FILE_NAME, &links)?.to_string_lossy())?;
			git::git_commit(&repo, &format!("Create link of `{}:{}` to `{}:{}`.", self.manifest.prefix, origin.object, destination.module(), destination.object))?;
		}
		Ok(())
	}

	pub fn delete_link(&mut self, repo: &Option<Repository>, origin: &Link, destination: &Link) -> Result<Links, ModuleError> {
		let repo_path: PathBuf = self.get_repository_path().unwrap_or_default();
		let dest_module_path: PathBuf = Module::add_paths(&repo_path, &PathBuf::from(&destination.path));
		let mut dest_module: Module = Module::read(&dest_module_path)?;
		dest_module.delete_inbound_link(&repo, origin, destination)?;
		self.deleted_outbound_link(&repo, origin, destination)?;
		self.read_links()
	}

	fn delete_inbound_link(&mut self, repo: &Option<Repository>, origin: &Link, destination: &Link) -> Result<(), ModuleError> {
		let repo: &Repository = Module::repo(repo)?;
		if let Some(inbound_links) = self.links.inbound_links.get_mut(&destination.object) {
			inbound_links.retain(|lnk| lnk != origin);
			if inbound_links.is_empty() {
				self.links.inbound_links.remove(&destination.object);
			}
			git::add_file(&repo, &mid::update_yml_file(&self.path, defs::OD_LINKS_FILE_NAME, &self.links)?.to_string_lossy())?;
			git::git_commit(&repo, &format!("Deleted link from `{}:{}` to `{}:{}`.", destination.module(), destination.object, self.manifest.prefix, origin.object))?;
		};
		Ok(())
	}
	
	fn deleted_outbound_link(&mut self, repo: &Option<Repository>, origin: &Link, destination: &Link) -> Result<(), ModuleError> {
		let repo: &Repository = Module::repo(repo)?;
		if let Some(outbound_links) = self.links.outbound_links.get_mut(&origin.object) {
			outbound_links.retain(|lnk| lnk != destination);
			if outbound_links.is_empty() {
				self.links.outbound_links.remove(&origin.object);
			}
			git::add_file(&repo, &mid::update_yml_file(&self.path, defs::OD_LINKS_FILE_NAME, &self.links)?.to_string_lossy())?;
			git::git_commit(&repo, &format!("Deleted link from `{}:{}` to `{}:{}`.", self.manifest.prefix, origin.object, destination.module(), destination.object))?;
		};
		Ok(())
	}

	pub fn read_links(&mut self) -> Result<Links, ModuleError> {
		self.links = mid::read_yml_file(&self.path, defs::OD_LINKS_FILE_NAME)?;
		Ok(self.links.clone())
	}

	fn tag_format(module: &Module, semver: &str) -> String {
		return format!("{}/{}", module.manifest.prefix.to_lowercase(), semver);
	}

	fn tag_from_semver(&self, semver: &str) -> String {
		return Module::tag_format(&self, semver);
	}

	fn prepare_object(&mut self, obj: &mut Object) -> Result<String, ModuleError> {
		let id: usize = self.save_object(defs::OD_DRAFT_FOLDER_NAME, obj)?;
		let filename: String = format!("{id}.yml");
		let origin = &self.path.join(defs::OD_DRAFT_FOLDER_NAME);
		let destination = &self.path.join(defs::OD_OBJS_FOLDER_NAME);
		if let Err(_) = mid::move_file(origin, destination, &filename) {
			mid::create_folder(&self.path, defs::OD_OBJS_FOLDER_NAME)?;
			mid::move_file(origin, destination, &filename)?;
		}
		Ok(destination.join(filename).to_string_lossy().into())
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
		obj.metadata = None;
		if obj.id() == 0 {
			obj.assign_id(self.get_next_available_id()?);
		}
		if let Err(_) = mid::create_yml_file(&self.path.join(folder), format!("{}.yml", obj.id()), &obj) {
			mid::create_folder(&self.path, folder)?;
			mid::create_yml_file(&self.path.join(folder), format!("{}.yml", obj.id()), &obj)?;
		}
		Ok(obj.id())
	}

	fn open_object(path: &PathBuf, id: usize) -> Result<Object, ModuleError> {
		Ok(mid::read_yml_file::<Object, _>(path, format!("{id}.yml"))?)
	}

	fn to_alpha_level(mut n: usize) -> String {
		let mut result = String::new();
		while n > 0 {
			n -= 1;
			let letter = ((n % 26) as u8 + b'a') as char;
			result.insert(0, letter);
			n /= 26;
		}
		result
	}

	fn sort_objects_by_hierarchy(objects: &[Object]) -> Vec<Object> {
		let mut by_id: HashMap<usize, &Object> = HashMap::new();
		let mut children: HashMap<usize, Vec<&Object>> = HashMap::new();

		for obj in objects {
			by_id.insert(obj.id(), obj);
			children.entry(obj.index_parent_id).or_default().push(obj);
		}

		for siblings in children.values_mut() {
			siblings.sort_by_key(|obj| obj.id());
		}

		let mut ordered = Vec::with_capacity(objects.len());

		fn dfs(current_id: usize, children: &HashMap<usize, Vec<&Object>>, ordered: &mut Vec<Object>) {
			if let Some(kids) = children.get(&current_id) {
				for &child in kids {
					ordered.push(child.clone());
					dfs(child.id(), children, ordered);
				}
			}
		}

		dfs(0, &children, &mut ordered);

		ordered
	}

	fn sort_objects(objects: Vec<Object>) -> Vec<Object> {
		let mut grouped: HashMap<usize, Vec<Object>> = HashMap::new();
		
		for obj in objects {
			grouped.entry(obj.index_parent_id).or_default().push(obj);
		}

		for (_parent, children) in grouped.iter_mut() {
			children.sort_by(|a, b| {
				let a_num = a.index_level.parse::<f64>().unwrap_or_default();
				let b_num = b.index_level.parse::<f64>().unwrap_or_default();
				a_num.partial_cmp(&b_num).unwrap()
			});
		}
		
		fn flatten(parent_id: usize, grouped: &HashMap<usize, Vec<Object>>, output: &mut Vec<Object>) {
			if let Some(children) = grouped.get(&parent_id) {
				for child in children {
					output.push(child.clone());
					flatten(child.id(), grouped, output);
				}
			}
		}

		let mut result: Vec<Object> = Vec::new();
		
		flatten(0, &grouped, &mut result);
		
		result
	}

	fn generate_levels(mut objects: Vec<Object>) -> Vec<Object> {
		// Mapa para rastrear contadores de cada parent_id
		let mut counters: HashMap<usize, usize> = HashMap::new();
		let mut alpha_counters: HashMap<usize, usize> = HashMap::new();

		// Mapa para guardar o "prefixo" do nível do pai
		let mut level_map: HashMap<usize, String> = HashMap::new();
		level_map.insert(0, "".to_string()); // topo

		for obj in &mut objects {
			let parent_level = level_map.get(&obj.index_parent_id).cloned().unwrap_or_default();

			if !obj.header.trim().is_empty() {
				// É título -> numeração numérica
				let count = counters.entry(obj.index_parent_id).and_modify(|c| *c += 1).or_insert(1);
				let new_level = if parent_level.is_empty() {
					format!("{}", count)
				} else {
					format!("{}.{}", parent_level, count)
				};
				obj.set_level(new_level.clone());
				level_map.insert(obj.id(), new_level.clone());
			} else {
				let count = alpha_counters.entry(obj.index_parent_id).and_modify(|c| *c += 1).or_insert(1);
				let letter = Module::to_alpha_level(*count);
				let new_level = if parent_level.is_empty() {
					format!("{}", letter)
				} else {
					format!("{}-{}", parent_level, letter)
				};
				obj.set_level(new_level);
				level_map.insert(obj.id(), parent_level.clone());
			}
		}

		objects
	}

	fn sort_by_level(objects: Vec<Object>) -> Vec<Object> {
		let sorted_objects: Vec<Object> = Module::sort_objects(objects);
		let leveled_objects: Vec<Object> = Module::generate_levels(sorted_objects);
		leveled_objects
	}

	fn repo(repo: &Option<Repository>) -> Result<&Repository, ModuleError> {
		repo.as_ref().ok_or(ModuleError::NoRepositoryInitialized)
	}
}
