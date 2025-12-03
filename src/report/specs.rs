use crate::diagnostics::LintViolation;
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
pub struct ReportSpecs {
    severity: ViolationSeverity,
    rule_id: String,
    message: String,
    labels: Vec<LabelSpecs>,
    notes: Vec<String>,
}

impl ReportSpecs {
    pub fn new(
        severity: &ViolationSeverity,
        rule_id: &str,
        message: String,
        labels: Vec<LabelSpecs>,
        notes: Vec<String>,
    ) -> Self {
        ReportSpecs {
            severity: severity.clone(),
            rule_id: rule_id.to_string(),
            message,
            labels,
            notes,
        }
    }

    pub fn from_violation(
        violation: &LintViolation,
        message: String,
        labels: Vec<LabelSpecs>,
        notes: Vec<String>,
    ) -> Self {
        ReportSpecs::new(
            violation.severity(),
            violation.rule_id(),
            message,
            labels,
            notes,
        )
    }
    pub fn severity(&self) -> &ViolationSeverity {
        &self.severity
    }

    pub fn code(&self) -> &str {
        &self.rule_id
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
