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

#[register_report(id = "INTER001")]
struct DiseaseConsistencyReport;

impl ReportFromContext for DiseaseConsistencyReport {
    fn from_context(_: &LinterContext) -> Result<Box<dyn RegisterableReport>, FromContextError> {
        Ok(Box::new(Self))
    }
}

impl CompileReport for DiseaseConsistencyReport {
    fn compile_report(
        &self,
        full_node: &DynamicNode,
        lint_violation: &LintViolation,
    ) -> ReportSpecs {
        let violation_ptr = lint_violation.at().first().unwrap().clone();
        let mut interpretation_ptr = violation_ptr.clone();

        let interpretation_id = full_node
            .value_at(interpretation_ptr.up().up())
            .expect("Interpretation should have been there")
            .get("id")
            .expect("Interpretation ID should have been there")
            .clone();

        ReportSpecs::new(DiagnosticSpec {
            severity: Severity::Warning,
            code: Self::RULE_ID.to_string(),
            message: format!("Found disease in interpretation {interpretation_id} that is not present in diseases section")
                .to_string(),
            labels: vec![LabelSpecs {
                style: LabelStyle::Primary,
                range: full_node.span_at(&violation_ptr).unwrap().clone(),
                message: String::default(),
            }],
            notes: vec![],
        })
    }
}
