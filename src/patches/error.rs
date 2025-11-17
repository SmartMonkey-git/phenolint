use crate::error::InitError;
use json_patch::PatchError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PatchingError {
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    InitError(#[from] InitError),
    #[error(transparent)]
    PatchError(#[from] PatchError),
}
