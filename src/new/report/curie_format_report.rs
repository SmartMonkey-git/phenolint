use crate::LinterContext;
use crate::diagnostics::specs::DiagnosticSpec;
use crate::diagnostics::{LintViolation, ReportSpecs};
use crate::error::RuleInitError;
use crate::new::node::Node;
use crate::new::report::report_registry::{
    CompileReport, RegisterableReport, ReportFromContext, RuleReport,
};
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
    fn compile_report(&self, value: &Node, lint_violation: &LintViolation) -> ReportSpecs {
        println!("Reached compilation of CurieFormatReport.");

        ReportSpecs::new(DiagnosticSpec {
            severity: Severity::Error,
            code: Self::RULE_ID.to_string(),
            message: "Reached Report Compiler".to_string(),
            labels: vec![],
            notes: vec![],
        })
    }
}
