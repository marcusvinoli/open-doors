use thiserror::Error;

#[derive(Debug, Error)]
pub enum MiddlewareError {
    #[error("IoError. {0}")]
    IoError(#[from] std::io::Error),
    #[error("Serialization/Deserialization error. {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),
}
