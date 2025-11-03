use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LintingError{

}


#[derive(Error, Debug)]
pub enum InstantiationError {
    #[error(transparent)]
    IO(std::io::Error),
    #[error(transparent)]
    Config(#[from] ConfigError),
}