use thiserror::Error;
use crate::core::middleware::error::MiddlewareError;

#[derive(Debug, Error)]
pub enum OpenDoorsError {
	#[error("GIT ERROR {0}")]
	GitError(#[from] crate::git::GitError),
	#[error("REPOSITORY ERROR {0}")]
	RespositoryError(#[from] RepositoryError),
	#[error("PROJECT ERROR: {0}")]
	ProjectError(#[from] ProjectError),
	#[error("FOLDER ERROR: {0}")]
	FolderError(#[from] FolderError),
	#[error("MODULE ERROR: {0}")]
	ModuleError(#[from] ModuleError),
	#[error("MIDDLEWARE ERROR: {0}")]
	TreeError(#[from] TreeError),
	#[error("GENERIC MIDDLEWARE ERROR: {0}")]
	MiddlewareError(#[from] MiddlewareError),
	#[error("CSV EXPORT ERROR: {0}")]
	CsvExporterError(#[from] csv::Error),
	#[error("XLSX EXPORT ERROR: {0}")]
	XlsxExporterError(#[from] xlsxwriter::XlsxError),
	#[error("GIT2 LIBRARY ERROR: {0}")]
	Git2Error(#[from] git2::Error),
	#[error("{0}")]
	GenericError(String)
}

impl serde::Serialize for OpenDoorsError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: serde::Serializer {
		serializer.serialize_str(self.to_string().as_ref())
	}
}

#[derive(Debug, Error)]
pub enum RepositoryError {
	#[error("Generic middleare error!{0}")]
	MiddlewareError(#[from] MiddlewareError),
	#[error("Not a git repository.")]
	InvalidRepository,
	#[error("The repository could not be created.")]
	RepositoryCouldNotBeCreated,
	#[error("The repository could not be created.")]
	RepositoryCouldNotBeOpen,
	#[error("Git Error: {0}")]
	GitError(#[from] crate::git::GitError),
	#[error("Git2 Error: {0}")]
	Git2Error(#[from] git2::Error),
	#[error("Unknown Error!")]
	UnknownError,
}

#[derive(Debug, Error)]
pub enum ProjectError {
	#[error("Generic middleware error! {0}")]
	MiddlewareError(#[from] MiddlewareError),
	#[error("The Directory is not a Project")]
	InvalidProjectDirectory,
	#[error("Fail creating project {0}: {1}")]
	ProjectCouldNotBeCreated(String, String),
	#[error("Fail reading project {0}: {1}")]
	ProjectCouldNotBeRead(String, String),
	#[error("Git2 Error! {0}")]
	Git2Error(#[from] git2::Error),
	#[error("No repository initialized!")]
	NoRepositoryInitialized,
	#[error("Unknown Error!")]
	UnknownError,
}

#[derive(Debug, Error)]
pub enum FolderError {
	#[error("Generic middleware error! {0}")]
	MiddlewareError(#[from] MiddlewareError),
	#[error("Git2 Error! {0}")]
	Git2Error(#[from] git2::Error),
	#[error("No repository initialized!")]
	NoRepositoryInitialized,
}

#[derive(Debug, Error)]
pub enum ModuleError {
	#[error("Generic middleware error! {0}")]
	MiddlewareError(#[from] MiddlewareError),
	#[error("The Directory is not a Module")]
	InvalidModuleDirectory,
	#[error("GIT ERROR {0}")]
	GitError(#[from] crate::git::GitError),
	#[error("Git2 Error! {0}")]
	Git2Error(#[from] git2::Error),
	#[error("Unknown Error!")]
	UnknownError,
	#[error("No repository initialized!")]
	NoRepositoryInitialized,
	#[error("No baseline tagged {0}.")]
	BaselineNotFound(String),
	#[error("Baseline not commited.")]
	BaselineNotCommited,
	#[error("Serialization error: {0}.")]
	SerdeYamlError(#[from] serde_yaml::Error)
}

#[derive(Debug, Error)] 
pub enum TreeError {
	#[error("Generic middleware error! {0}")]
	MiddlewareError(#[from] MiddlewareError),
}
