use codespan_reporting::diagnostic::{Diagnostic, Label};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum ViolationSeverity {
    /// Indicates that the violation will likely break other applications
    Error,
    /// Indicates that the violation is unexpected but not breaking
    Warning,
    /// Something has been violated that is not breaking, not unexpected but needs some attention
    Help,
}

impl ViolationSeverity {
    pub(crate) fn as_codespan_diagnostic(&self) -> Diagnostic<usize> {
        match self {
            ViolationSeverity::Error => Diagnostic::error(),
            ViolationSeverity::Warning => Diagnostic::warning(),
            ViolationSeverity::Help => Diagnostic::help(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LabelPriority {
    /// Primary message of the report
    Primary,
    /// Any secondary message of the report
    Secondary,
}

impl LabelPriority {
    pub(crate) fn as_codespan_label<FileID>(
        &self,
        file_id: FileID,
        range: &Range<usize>,
    ) -> Label<FileID> {
        match self {
            LabelPriority::Primary => Label::primary(file_id, range.clone()),
            LabelPriority::Secondary => Label::secondary(file_id, range.clone()),
        }
    }
}
