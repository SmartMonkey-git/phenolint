use crate::diagnostics::LintReport;
use crate::json::error::JsonEditError;
use config::ConfigError;
use thiserror::Error;

pub struct LintResult {
    pub report: LintReport,
    pub error: Option<LinterError>,
}

impl LintResult {
    pub fn new(report: LintReport, error: Option<LinterError>) -> Self {
        Self { report, error }
    }

    pub fn ok(report: LintReport) -> Self {
        Self::new(report, None)
    }
    pub fn err(error: LinterError) -> Self {
        Self::new(LintReport::default(), Some(error))
    }

    pub fn into_result(self) -> Result<LintReport, LinterError> {
        match self.error {
            Some(err) => Err(err),
            None => Ok(self.report),
        }
    }
}

#[derive(Error, Debug)]
pub enum LinterError {
    #[error("Can't patch Phenopacket {0}")]
    PatchingError(PatchingError),
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
