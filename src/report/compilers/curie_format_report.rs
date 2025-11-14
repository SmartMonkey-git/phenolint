#![allow(unused)]
use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::RuleInitError;
use crate::report::specs::{DiagnosticSpec, ReportSpecs};
use crate::report::traits::{CompileReport, RegisterableReport, ReportFromContext, RuleReport};
use crate::tree::node::Node;
use crate::tree::pointer::Pointer;
use codespan_reporting::diagnostic::Severity;
use phenolint_macros::register_report;

#[register_report(id = "CURIE001")]
struct CurieFormatReport;

impl ReportFromContext for CurieFormatReport {
    fn from_context(context: &LinterContext) -> Result<Box<dyn RegisterableReport>, RuleInitError> {
        Ok(Box::new(CurieFormatReport))
    }
}

impl CompileReport for CurieFormatReport {
    #[allow(unused)]
    fn compile_report(&self, value: &Node, lint_violation: &LintViolation) -> ReportSpecs {
        println!("Reached compilation of CurieFormatReport.");
        let curie = value.value(Pointer::new("id"));
        ReportSpecs::new(DiagnosticSpec {
            severity: Severity::Error,
            code: Self::RULE_ID.to_string(),
            message: format!("CURIE formatted wrong: {}", curie),
            labels: vec![],
            notes: vec![],
        })
    }
}
