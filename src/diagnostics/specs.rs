use codespan_reporting::diagnostic::{LabelStyle, Severity};
use std::ops::Range;
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
/*
fn a() {
    let diagnostic = Diagnostic::error()
        .with_message("`case` clauses have incompatible types")
        .with_code("E0308")
        .with_labels(vec![
            Label::primary(file_id, 328..331).with_message("expected `String`, found `Nat`"),
            Label::secondary(file_id, 211..331)
                .with_message("`case` clauses have incompatible types"),
            Label::secondary(file_id, 258..268)
                .with_message("this is found to be of type `String`"),
            Label::secondary(file_id, 284..290)
                .with_message("this is found to be of type `String`"),
            Label::secondary(file_id, 306..312)
                .with_message("this is found to be of type `String`"),
            Label::secondary(file_id, 186..192).with_message("expected type `String` found here"),
        ])
        .with_notes(vec![unindent::unindent(
            "
            expected type `String`
                found type `Nat`
        ",
        )]);
}
*/
