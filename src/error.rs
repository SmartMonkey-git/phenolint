use crate::json::error::JsonEditError;
use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinterError {
    #[error(transparent)]
    PatchingError(#[from] PatchingError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum InstantiationError {
    #[error(transparent)]
    IO(std::io::Error),
    #[error(transparent)]
    Config(#[from] ConfigError),
}

#[derive(Error, Debug)]
pub enum PatchingError {
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    JsonEditError(#[from] JsonEditError),
}

pub enum RuleInitError {
    NeedsHPO,
}
