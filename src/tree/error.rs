use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeRepositoryError {
    #[error("Cant Reinstantiate Node at '{0}' with type '{1}'.")]
    CantReinstantiateNode(String, String),
}
