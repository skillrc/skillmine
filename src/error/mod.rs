use thiserror::Error;

#[derive(Error, Debug)]
pub enum SkillmineError {
    #[error("Git error: {0}")]
    GitError(String),

    #[error("Registry error: {0}")]
    RegistryError(String),

    #[error("Installation error: {0}")]
    InstallationError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SkillmineError>;
