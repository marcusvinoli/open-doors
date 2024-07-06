use thiserror::Error;

#[derive(Debug, Error)]
pub enum GitError {
    #[error("Executor error.")]
    ExecutorError(#[from] std::io::Error),
    #[error("Git binary not found.")]
    GitBinaryNotFound,
    #[error("Not a git repository.")]
    NotAGitRepository,
    #[error("Repository not Empty.")]
    RepositoryNotEmpty,
}
