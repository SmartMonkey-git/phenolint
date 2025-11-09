use codespan_reporting::diagnostic::{LabelStyle, Severity};
use std::ops::Range;
#[derive(Debug, Clone, PartialEq)]
pub struct LabelSpecs {
    pub style: LabelStyle,
    pub range: Range<usize>,
    pub message: String,
}

impl LabelSpecs {
    pub fn style(&self) -> &LabelStyle {
        &self.style
    }

    pub fn range(&self) -> &Range<usize> {
        &self.range
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticSpec {
    pub severity: Severity,
    pub code: Option<String>,
    pub message: String,
    pub labels: Vec<LabelSpecs>,
    pub notes: Vec<String>,
}

impl DiagnosticSpec {
    pub fn severity(&self) -> &Severity {
        &self.severity
    }

    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn labels(&self) -> &[LabelSpecs] {
        &self.labels
    }

    pub fn notes(&self) -> &[String] {
        &self.notes
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReportSpecs {
    diagnostic_spec: DiagnosticSpec,
}

impl ReportSpecs {
    pub fn new(report: DiagnosticSpec) -> Self {
        Self {
            diagnostic_spec: report,
        }
    }
    pub fn diagnostics(&self) -> &DiagnosticSpec {
        &self.diagnostic_spec
    }

    pub fn into_inner(self) -> DiagnosticSpec {
        self.diagnostic_spec
    }
}
