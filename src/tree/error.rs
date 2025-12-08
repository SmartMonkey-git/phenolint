use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeRepositoryError {
    #[error("Cant Reinstantiate Node at '{0}' and type '{1}'.")]
    CantReinstantiateNode(String, String),
}
