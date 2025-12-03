use codespan_reporting::diagnostic::{Diagnostic, Label};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum ViolationSeverity {
    /// Critical issues that will cause failures (e.g. runtime crashes,
    /// or contract violations that break dependent code)
    Error,
    /// Problematic patterns that should be fixed but won't cause immediate failure
    Warning,
    /// Suggestions for improvement or style violations that don't affect correctness
    Info,
}

impl ViolationSeverity {
    pub(crate) fn as_codespan_diagnostic(&self) -> Diagnostic<usize> {
        match self {
            ViolationSeverity::Error => Diagnostic::error(),
            ViolationSeverity::Warning => Diagnostic::warning(),
            ViolationSeverity::Info => Diagnostic::help(),
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
