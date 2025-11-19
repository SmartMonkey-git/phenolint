#![allow(unused)]
use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::report::report_registration::ReportRegistration;
use crate::report::specs::{DiagnosticSpec, LabelSpecs, ReportSpecs};
use crate::report::traits::{CompileReport, RegisterableReport, ReportFromContext, RuleReport};
use crate::tree::node::Node;
use crate::tree::pointer::Pointer;
use codespan_reporting::diagnostic::{LabelStyle, Severity};
use phenolint_macros::register_report;

#[register_report(id = "CURIE001")]
struct CurieFormatReport;

impl ReportFromContext for CurieFormatReport {
    fn from_context(
        context: &LinterContext,
    ) -> Result<Box<dyn RegisterableReport>, FromContextError> {
        Ok(Box::new(CurieFormatReport))
    }
}

impl CompileReport for CurieFormatReport {
    #[allow(unused)]
    fn compile_report(&self, node: &Node, lint_violation: &LintViolation) -> ReportSpecs {
        let curie = node.value(Pointer::new("id"));
        ReportSpecs::new(DiagnosticSpec {
            severity: Severity::Error,
            code: Self::RULE_ID.to_string(),
            message: format!("CURIE formatted wrong: {}", curie),
            labels: vec![LabelSpecs {
                style: LabelStyle::Primary,
                range: node.span(&node.pointer).unwrap().clone(),
                message: String::default(),
            }],
            notes: vec![],
        })
    }
}
