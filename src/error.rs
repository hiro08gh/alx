use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlxError {
    #[error("Alias '{0}' already exists")]
    AliasExists(String),

    #[error("Alias '{0}' not found")]
    AliasNotFound(String),

    #[error("Invalid alias name: {0}")]
    InvalidAliasName(String),

    #[error("Invalid command: {0}")]
    InvalidCommand(String),

    #[error("Config file error: {0}")]
    ConfigError(String),

    #[error("Shell detection failed")]
    ShellDetectionFailed,

    #[error("Shell not supported: {0}")]
    UnsupportedShell(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] toml::de::Error),

    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] toml::ser::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AlxError>;
