use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::helper::non_empty_vec::NonEmptyVec;
use crate::linter_context::LinterContext;
use crate::report::report_registration::ReportRegistration;
use crate::report::specs::{DiagnosticSpec, LabelSpecs, ReportSpecs};
use crate::report::traits::RuleReport;
use crate::report::traits::{CompileReport, RegisterableReport, ReportFromContext};
use crate::rules::rule_registration::RuleRegistration;
use crate::rules::traits::RuleMetaData;
use crate::rules::traits::{LintRule, RuleCheck, RuleFromContext};
use crate::tree::node_repository::List;
use crate::tree::traits::Node;
use codespan_reporting::diagnostic::{LabelStyle, Severity};
use phenolint_macros::{register_report, register_rule};
use phenopackets::schema::v2::core::OntologyClass;
use regex::Regex;

/// ### CURIE001
/// ## What it does
/// Identifies CURIE ID that are not formatted correctly.
///
/// ## Why is this bad?
/// Matching incorrectly formatted ID's back to their original sources can cause problems, when
/// computationally using the phenopacket.
#[derive(Debug)]
#[register_rule(id = "CURIE001")]
pub struct CurieFormatRule {
    regex: Regex,
}

impl RuleFromContext for CurieFormatRule {
    fn from_context(_: &LinterContext) -> Result<Box<dyn LintRule>, FromContextError> {
        Ok(Box::new(CurieFormatRule {
            regex: Regex::new("^[A-Z][A-Z0-9_]+:[A-Za-z0-9_]+$").expect("Invalid regex"),
        }))
    }
}

impl RuleCheck for CurieFormatRule {
    type Data<'a> = List<'a, OntologyClass>;

    fn check(&self, data: Self::Data<'_>) -> Vec<LintViolation> {
        let mut violations = vec![];

        for node in data.0.iter() {
            if !self.regex.is_match(&node.inner.id) {
                let mut ptr = node.pointer().clone();
                ptr.down("id");

                violations.push(LintViolation::new(
                    LintRule::rule_id(self),
                    NonEmptyVec::with_single_entry(ptr),
                ))
            }
        }

        violations
    }
}

#[register_report(id = "CURIE001")]
struct CurieFormatReport;

impl ReportFromContext for CurieFormatReport {
    fn from_context(_: &LinterContext) -> Result<Box<dyn RegisterableReport>, FromContextError> {
        Ok(Box::new(CurieFormatReport))
    }
}

impl CompileReport for CurieFormatReport {
    fn compile_report(&self, full_node: &dyn Node, lint_violation: &LintViolation) -> ReportSpecs {
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
                span: full_node.span_at(&violation_ptr).unwrap().clone(),
                message: String::default(),
            }],
            notes: vec![],
        })
    }
}
