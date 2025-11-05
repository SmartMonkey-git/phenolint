use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinterError {

}


#[derive(Error, Debug)]
pub enum InstantiationError {
    #[error(transparent)]
    IO(std::io::Error),
    #[error(transparent)]
    Config(#[from] ConfigError),
}


#[derive(Error, Debug)]
pub enum FixingError {
    #[error("TODO")]
    SomeError,
}