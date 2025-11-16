use crate::error::InitError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PatchingError {
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    InitError(#[from] InitError),
}
