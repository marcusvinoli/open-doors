mod definitions;
use std::path::PathBuf;
use definitions as defs;
use git2::{self, Error, ErrorClass, ErrorCode, IndexAddOption, ObjectType, RemoteCallbacks, Repository};

use super::middleware;

type Result<T> = std::result::Result<T, Error>;

pub fn init(path: &str) -> Result<Repository> {
	Repository::init(path)
}

pub fn create_ignore_rules(repo: &Repository, rules: &[&str]) -> Result<()> {
	let mut base_repo = repo.path().to_path_buf();
	base_repo.pop();
	let _ = middleware::create_file(&base_repo, ".gitignore");
	for rule in rules {
		repo.add_ignore_rule(*rule)?;
	}
	Ok(())
}

pub fn add_remote(repo: &Repository, name: &str, url: &str) -> Result<()> {
	repo.remote(name, url)?;
	Ok(())
}

pub fn add_file(repo: &Repository, path: &str) -> Result<()> {
	let mut repo_path: PathBuf = PathBuf::from(repo.path());
	repo_path.pop();
	let add_path: PathBuf = PathBuf::from(path);
	let path = if let Ok(relative) = add_path.strip_prefix(&repo_path) {
		relative
	} else {
		add_path.as_path()
	};
	let mut index = repo.index()?;
	index.add_path(&PathBuf::from(path))?;
	index.write()?;
	Ok(())
}

pub  fn  add_folder(repo: &Repository, path: &str) -> Result<()> {
	let mut repo_path: PathBuf = PathBuf::from(repo.path());
	repo_path.pop();
	let add_path: PathBuf = PathBuf::from(path);
	let path = if let Ok(relative) = add_path.strip_prefix(&repo_path) {
		relative
	} else {
		add_path.as_path()
	};
	let mut index = repo.index()?;
	index.add_all([path].iter(), IndexAddOption::DEFAULT, None)?;
	index.write()?;
	Ok(())
}

pub fn git_commit(repo: &Repository, msg: &str) -> Result<()> {
	let mut index = repo.index()?;
	let signature = repo.signature()?;
	let oid = index.write_tree()?;
	let tree = repo.find_tree(oid)?;
	let head = match repo.head() {
		Ok(reference) => Some(reference.target().unwrap()),
		Err(_) => None
	};
	let parent_commit = if let Some(head_oid) = head {
		Some(repo.find_commit(head_oid)?)
	} else {
		None
	};
	let parents = parent_commit.as_ref().map_or_else(|| vec![], |p| vec![p]);
	repo.commit(Some(defs::HEAD), &signature, &signature, msg, &tree, &parents)?;
	Ok(())
}

pub fn git_push(repo: &Repository) -> Result<()> {
	let mut remote = repo.find_remote(defs::REMOTE_NAME)?;
	let refspec = "refs/heads/master:refs/heads/master";
	remote.push(&[refspec], None)?;
	Ok(())
}

pub fn git_pull(repo: &git2::Repository) -> Result<()> {
	let mut callbacks = RemoteCallbacks::new();
	callbacks.credentials(|_url, _username_from_url, _allowed_types| {
		git2::Cred::ssh_key_from_agent("git")
	});

	let mut fetch_options = git2::FetchOptions::new();
	fetch_options.remote_callbacks(callbacks);

	let mut remote = repo.find_remote(defs::REMOTE_NAME)?;
	remote.fetch(&[defs::DEFAULT_BRANCH], Some(&mut fetch_options), None)?;

	let fetch_head_ref = format!("refs/remotes/{}/{}", defs::REMOTE_NAME, defs::DEFAULT_BRANCH);
	let fetch_head = repo.find_reference(&fetch_head_ref)?.peel_to_commit()?;

	let local_branch = repo.find_branch(defs::DEFAULT_BRANCH, git2::BranchType::Local)?;
	let local_commit = local_branch.get().peel_to_commit()?;

	let mut index = repo.merge_commits(&local_commit, &fetch_head, None)?;
	if index.has_conflicts() {
		return Err(Error::new(ErrorCode::Conflict, ErrorClass::Worktree, "There are some conflicts!"));
	}

	let tree = repo.find_tree(index.write_tree_to(&repo)?)?;
	let signature = repo.signature()?;

	repo.commit(Some(defs::HEAD), &signature, &signature, &format!("Automatic merge"), &tree, &[&local_commit, &fetch_head])?;

	Ok(())
}

pub fn create_tag(repo: &git2::Repository, version: &str, msg: &str) -> Result<String> {
	let obj = repo.head()?.peel(ObjectType::Commit)?;
	let signature = repo.signature()?;
	Ok(repo.tag(&version, &obj, &signature, &msg, false)?.to_string())
}

pub fn push_tag(repo: &git2::Repository) -> Result<()> {
    let mut remote = repo.find_remote("origin")?;
    remote.push(&["refs/heads/main", "refs/tags/*"], None)?;
    Ok(())
}
