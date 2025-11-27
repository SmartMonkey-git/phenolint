use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::report::report_registration::ReportRegistration;
use crate::report::specs::{DiagnosticSpec, LabelSpecs, ReportSpecs};
use crate::report::traits::{CompileReport, RuleReport};
use crate::report::traits::{RegisterableReport, ReportFromContext};
use crate::tree::node::DynamicNode;
use crate::tree::pointer::Pointer;
use codespan_reporting::diagnostic::{LabelStyle, Severity};
use phenolint_macros::register_report;

#[register_report(id = "INTER002")]
pub struct CuriesHaveResourcesReport;

impl ReportFromContext for CuriesHaveResourcesReport {
    fn from_context(_: &LinterContext) -> Result<Box<dyn RegisterableReport>, FromContextError> {
        Ok(Box::new(Self))
    }
}

impl CompileReport for CuriesHaveResourcesReport {
    fn compile_report(
        &self,
        full_node: &DynamicNode,
        lint_violation: &LintViolation,
    ) -> ReportSpecs {
        let metadata_ptr = Pointer::new("/metaData/resources");
        let ds = DiagnosticSpec {
            severity: Severity::Error,
            code: self.rule_id(),
            message: "An ontology class needs a resource".to_string(),
            labels: vec![
                LabelSpecs {
                    style: LabelStyle::Primary,
                    range: full_node
                        .spans
                        .get(&metadata_ptr)
                        .cloned()
                        .expect("It should be here"),
                    message: "This ontology class ...".to_string(),
                },
                LabelSpecs {
                    style: LabelStyle::Secondary,
                    range: full_node
                        .spans
                        .get(lint_violation.at().first().expect("Should be there"))
                        .cloned()
                        .expect("Should be there"),
                    message: "... should have a resource here".to_string(),
                },
            ],
            notes: vec![
                "Phenopacket Schema prescribes that all ontology classes need a resource to document the version of the used ontology, or to support CURIE -> IRI expansion.".to_string(),
            ],
        };
        ReportSpecs::new(ds)
    }
}
