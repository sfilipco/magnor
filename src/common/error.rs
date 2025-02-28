use thiserror::Error;

#[derive(Error, Debug)]
pub enum MagnorError {
    #[error("Query parsing error: {0}")]
    QueryParseError(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Schema error: {0}")]
    SchemaError(String),

    #[error("Storage error: {0}")]
    StorageError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, MagnorError>;
