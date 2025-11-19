use crate::diagnostics::LintReport;
use crate::patches::error::PatchingError;
use config::ConfigError;
use jsonschema::error::ValidationErrorKind;
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
    #[error("Invalid Phenopacket at: '{path}'. Reason: '{reason}'")]
    InvalidPhenopacket { path: String, reason: String },
}

pub(crate) fn validation_error_to_string(kind: &ValidationErrorKind) -> String {
    match kind {
        ValidationErrorKind::AdditionalItems { limit } => {
            format!("Array contains more than {} items", limit)
        }
        ValidationErrorKind::AdditionalProperties { unexpected } => {
            format!("Unexpected properties: {}", unexpected.join(", "))
        }
        ValidationErrorKind::AnyOf { .. } => {
            "Value is not valid under any of the 'anyOf' schemas".to_string()
        }
        ValidationErrorKind::BacktrackLimitExceeded { .. } => {
            "Pattern matching backtrack limit exceeded".to_string()
        }
        ValidationErrorKind::Constant { expected_value } => {
            format!("Value doesn't match expected constant: {}", expected_value)
        }
        ValidationErrorKind::Contains => {
            "Array doesn't contain items conforming to the specified schema".to_string()
        }
        ValidationErrorKind::ContentEncoding { content_encoding } => {
            format!("Invalid content encoding: {}", content_encoding)
        }
        ValidationErrorKind::ContentMediaType { content_media_type } => {
            format!("Invalid content media type: {}", content_media_type)
        }
        ValidationErrorKind::Custom { message } => message.to_string(),
        ValidationErrorKind::Enum { options } => {
            format!(
                "Value doesn't match any of the allowed options: {}",
                options
            )
        }
        ValidationErrorKind::ExclusiveMaximum { limit } => {
            format!("Value must be less than {}", limit)
        }
        ValidationErrorKind::ExclusiveMinimum { limit } => {
            format!("Value must be greater than {}", limit)
        }
        ValidationErrorKind::FalseSchema => "Schema is false, all values are invalid".to_string(),
        ValidationErrorKind::Format { format } => {
            format!("Value doesn't match required format: {}", format)
        }
        ValidationErrorKind::FromUtf8 { .. } => "Invalid UTF-8 in base64 encoded data".to_string(),
        ValidationErrorKind::MaxItems { limit } => {
            format!("Array has more than {} items", limit)
        }
        ValidationErrorKind::Maximum { limit } => {
            format!("Value must be at most {}", limit)
        }
        ValidationErrorKind::MaxLength { limit } => {
            format!("String is longer than {} characters", limit)
        }
        ValidationErrorKind::MaxProperties { limit } => {
            format!("Object has more than {} properties", limit)
        }
        ValidationErrorKind::MinItems { limit } => {
            format!("Array has fewer than {} items", limit)
        }
        ValidationErrorKind::Minimum { limit } => {
            format!("Value must be at least {}", limit)
        }
        ValidationErrorKind::MinLength { limit } => {
            format!("String is shorter than {} characters", limit)
        }
        ValidationErrorKind::MinProperties { limit } => {
            format!("Object has fewer than {} properties", limit)
        }
        ValidationErrorKind::MultipleOf { multiple_of } => {
            format!("Value is not a multiple of {}", multiple_of)
        }
        ValidationErrorKind::Not { schema } => {
            format!("Value matches negated schema: {}", schema)
        }
        ValidationErrorKind::OneOfMultipleValid { .. } => {
            "Value is valid under multiple 'oneOf' schemas (must match exactly one)".to_string()
        }
        ValidationErrorKind::OneOfNotValid { .. } => {
            "Value is not valid under any 'oneOf' schema".to_string()
        }
        ValidationErrorKind::Pattern { pattern } => {
            format!("Value doesn't match required pattern: {}", pattern)
        }
        ValidationErrorKind::PropertyNames { .. } => {
            "Object property names are invalid".to_string()
        }
        ValidationErrorKind::Required { property } => {
            format!("Required property missing: {}", property)
        }
        ValidationErrorKind::Type { kind } => {
            format!("Invalid type, expected: {:?}", kind)
        }
        ValidationErrorKind::UnevaluatedItems { unexpected } => {
            format!("Unevaluated items: {}", unexpected.join(", "))
        }
        ValidationErrorKind::UnevaluatedProperties { unexpected } => {
            format!("Unevaluated properties: {}", unexpected.join(", "))
        }
        ValidationErrorKind::UniqueItems => "Array contains duplicate items".to_string(),
        ValidationErrorKind::Referencing(err) => {
            format!("Schema reference resolution error: {:?}", err)
        }
    }
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
