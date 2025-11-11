use crate::json::Pointer;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EditError {
    #[error("'{0}' is not implemented yet")]
    NotImplemented(String),
    #[error("Position '{0}' is not valid")]
    InvalidPosition(Pointer),
    #[error("Editor tried to insert at root")]
    RootInsert,
    #[error("Expected Array at '{0}'")]
    ExpectedArrayOrObject(Pointer),
    #[error("Array index out of bounce '{0}' is not valid")]
    ArrayIndexOutOfBounce(Pointer),
    #[error("'{0}' Is not a value")]
    NotAJsonValue(String),
    #[error(transparent)]
    ExportError(#[from] serde_json::error::Error),
}
