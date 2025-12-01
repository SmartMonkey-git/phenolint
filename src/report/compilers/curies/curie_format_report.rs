use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::report::report_registration::ReportRegistration;
use crate::report::specs::{DiagnosticSpec, LabelSpecs, ReportSpecs};
use crate::report::traits::{CompileReport, RegisterableReport, ReportFromContext, RuleReport};
use crate::tree::node::DynamicNode;
use crate::tree::traits::Node;
use codespan_reporting::diagnostic::{LabelStyle, Severity};
use phenolint_macros::register_report;

#[register_report(id = "CURIE001")]
struct CurieFormatReport;

impl ReportFromContext for CurieFormatReport {
    fn from_context(_: &LinterContext) -> Result<Box<dyn RegisterableReport>, FromContextError> {
        Ok(Box::new(CurieFormatReport))
    }
}

impl CompileReport for CurieFormatReport {
    fn compile_report(
        &self,
        full_node: &DynamicNode,
        lint_violation: &LintViolation,
    ) -> ReportSpecs {
        let violation_ptr = lint_violation.at().first().unwrap().clone();
        let curie = full_node
            .value_at(&violation_ptr)
            .expect("CURIE should exist");

        ReportSpecs::new(DiagnosticSpec {
            severity: Severity::Error,
            code: Self::RULE_ID.to_string(),
            message: format!("CURIE formatted wrong: {}", curie),
            labels: vec![LabelSpecs {
                style: LabelStyle::Primary,
                range: full_node.span_at(&violation_ptr).unwrap().clone(),
                message: String::default(),
            }],
            notes: vec![],
        })
    }
}
