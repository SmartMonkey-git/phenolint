use crate::diagnostics::LintReport;
use crate::patches::error::PatchingError;
use config::ConfigError;
use prost::{DecodeError, EncodeError};
use saphyr::ScanError;
use std::string::FromUtf8Error;
use thiserror::Error;

/// A result type that combines a lint report with an optional error.
///
/// This struct provides a flexible way to handle linting operations that may
/// produce both partial results (in the form of a `LintReport`) and errors.
/// Unlike the standard `Result` type, `LintResult` allows carrying both a
/// report and an error simultaneously, which is useful when you want to
/// return diagnostic information even in the presence of errors.
#[derive(Debug)]
pub struct LintResult {
    pub report: LintReport,
    pub error: Option<LinterError>,
}

impl LintResult {
    pub fn partial(report: LintReport, error: LinterError) -> Self {
        Self {
            report,
            error: Some(error),
        }
    }

    pub fn ok(report: LintReport) -> Self {
        Self {
            report,
            error: None,
        }
    }
    pub fn err(error: LinterError) -> Self {
        Self::partial(LintReport::default(), error)
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
    PatchingError(#[from] PatchingError),
    #[error(transparent)]
    InitError(#[from] InitError),
    #[error(transparent)]
    ParsingError(#[from] ParsingError),
}

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Unable to parse input file")]
    Unparseable,
    #[error(transparent)]
    StringParsing(#[from] FromUtf8Error),
    #[error(transparent)]
    DecodeError(#[from] DecodeError),
    #[error(transparent)]
    EncodeError(#[from] EncodeError),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    YamlSpanError(#[from] ScanError),
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
}

#[derive(Error, Debug)]
pub enum InitError {
    #[error(transparent)]
    IO(std::io::Error),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    ParsingError(#[from] ParsingError),
}

#[derive(Debug, Error)]
pub enum FromContextError {
    #[error(
        "Rule '{rule_ids}'  was configured, but needs the {ontology}. {ontology} not found or not configured."
    )]
    NeedsOntology { rule_ids: String, ontology: String },
}
