use thiserror::Error;

#[derive(Error, Debug)]
pub enum SkillmineError {
    #[error("Git error: {0}")]
    Git(String),

    #[error("Registry error: {0}")]
    Registry(String),

    #[allow(dead_code)]
    #[error("Installation error: {0}")]
    Installation(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Unsupported error: {0}")]
    Unsupported(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SkillmineError>;
