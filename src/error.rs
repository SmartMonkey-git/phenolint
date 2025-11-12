use crate::diagnostics::LintReport;
use crate::json::error::JsonEditError;
use config::ConfigError;
use thiserror::Error;

/// A result type that combines a lint report with an optional error.
///
/// This struct provides a flexible way to handle linting operations that may
/// produce both partial results (in the form of a `LintReport`) and errors.
/// Unlike the standard `Result` type, `LintResult` allows carrying both a
/// report and an error simultaneously, which is useful when you want to
/// return diagnostic information even in the presence of errors.
pub struct LintResult {
    pub report: LintReport,
    pub error: Option<LinterError>,
}

impl LintResult {
    pub fn partial(report: LintReport, error: Option<LinterError>) -> Self {
        Self { report, error }
    }

    pub fn ok(report: LintReport) -> Self {
        Self::partial(report, None)
    }
    pub fn err(error: LinterError) -> Self {
        Self::partial(LintReport::default(), Some(error))
    }

    pub fn report(&self) -> &LintReport {
        &self.report
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
    #[error(transparent)]
    InitError(#[from] InitError),
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error(transparent)]
    IO(std::io::Error),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum PatchingError {
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    JsonEditError(#[from] JsonEditError),
    #[error(transparent)]
    InitError(#[from] InitError),
}

pub enum RuleInitError {
    NeedsHPO,
}
