use crate::report::enums::{LabelPriority, ViolationSeverity};
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct LabelSpecs {
    style: LabelPriority,
    span: Range<usize>,
    message: String,
}

impl LabelSpecs {
    pub fn new(style: LabelPriority, span: Range<usize>, message: String) -> Self {
        LabelSpecs {
            style,
            span,
            message,
        }
    }
    pub fn style(&self) -> &LabelPriority {
        &self.style
    }

    pub fn range(&self) -> &Range<usize> {
        &self.span
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiagnosticSpec {
    severity: ViolationSeverity,
    code: String,
    message: String,
    labels: Vec<LabelSpecs>,
    notes: Vec<String>,
}

impl DiagnosticSpec {
    pub fn new(
        severity: ViolationSeverity,
        code: String,
        message: String,
        labels: Vec<LabelSpecs>,
        notes: Vec<String>,
    ) -> Self {
        DiagnosticSpec {
            severity,
            code,
            message,
            labels,
            notes,
        }
    }
    pub fn severity(&self) -> &ViolationSeverity {
        &self.severity
    }

    pub fn code(&self) -> &str {
        &self.code
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

/// TODO: Temp default implementation. Should not have one.
impl Default for ReportSpecs {
    fn default() -> Self {
        let diag = DiagnosticSpec {
            severity: ViolationSeverity::Help,
            code: "None".to_string(),
            message: "I'm the default Report. You forgot to implement me.".to_string(),
            labels: vec![],
            notes: vec![],
        };
        ReportSpecs::new(diag)
    }
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
