use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpenDoorsError {
    #[error("GIT ERROR {0}")]
    GitError(#[from] crate::git::GitError),
    #[error("REPOSITORY ERROR {0}")]
    RespositoryError(#[from] RepositoryError),
    #[error("PROJECT ERROR: {0}")]
    ProjectError(#[from] ProjectError),
    #[error("MODULE ERROR: {0}")]
    ModuleError(#[from] ModuleError),
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
    #[error("Generic IO error!")]
    GenericIOError(#[from] std::io::Error),
    #[error("Parsing failure. {0}")]
    SerDeGenericFailure(#[from] serde_yaml::Error),
    #[error("Not a git repository.")]
    InvalidRepository,
    #[error("The repository could not be created.")]
    RepositoryCouldNotBeCreated,
    #[error("The repository could not be created.")]
    RepositoryCouldNotBeOpen,
    #[error("Git Error: {0}")]
    GitError(#[from] crate::git::GitError),
    #[error("Unknown Error!")]
    UnknownError,
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Generic IO error!")]
    GenericIOError(#[from] std::io::Error),
    #[error("Parsing failure. {0}")]
    SerDeGenericFailure(#[from] serde_yaml::Error),
    #[error("The Directory is not a Project")]
    InvalidProjectDirectory,
    #[error("Fail creating project {0}: {1}")]
    ProjectCouldNotBeCreated(String, String),
    #[error("Fail reading project {0}: {1}")]
    ProjectCouldNotBeRead(String, String),
    #[error("Unknown Error!")]
    UnknownError,
}

#[derive(Debug, Error)]
pub enum ModuleError {
    #[error("Generic IO error!")]
    GenericIOError(#[from] std::io::Error),
    #[error("Parsing failure. {0}")]
    SerDeGenericFailure(#[from] serde_yaml::Error),
    #[error("The Directory is not a Module")]
    InvalidModuleDirectory,
    #[error("Unknown Error!")]
    UnknownError,
}