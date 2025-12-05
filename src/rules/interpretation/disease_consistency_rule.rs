use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::helper::non_empty_vec::NonEmptyVec;
use crate::patches::enums::PatchInstruction;
use crate::patches::patch::Patch;
use crate::patches::patch_registration::PatchRegistration;
use crate::patches::traits::RulePatch;
use crate::patches::traits::{CompilePatches, PatchFromContext, RegisterablePatch};
use crate::report::enums::{LabelPriority, ViolationSeverity};
use crate::report::report_registration::ReportRegistration;
use crate::report::specs::{LabelSpecs, ReportSpecs};
use crate::report::traits::{CompileReport, RegisterableReport, ReportFromContext, RuleReport};
use crate::rules::rule_registration::RuleRegistration;
use crate::rules::traits::RuleMetaData;
use crate::rules::traits::{LintRule, RuleCheck, RuleFromContext};
use crate::tree::node_repository::List;
use crate::tree::pointer::Pointer;
use crate::tree::traits::{LocatableNode, Node};
use phenolint_macros::{register_patch, register_report, register_rule};
use phenopackets::schema::v2::core::{Diagnosis, Disease, OntologyClass};
use serde_json::Value;

#[derive(Debug, Default)]
/// ### INTER001
/// ## What it does
/// Checks if all diseases found in the interpretation section are also present in the diseases section.
///
/// ## Why is this bad?
/// It is expected that the disease section contains all diseases associated with a patient.
#[register_rule(id = "INTER001")]
pub struct DiseaseConsistencyRule;

impl RuleFromContext for DiseaseConsistencyRule {
    fn from_context(_: &LinterContext) -> Result<Box<dyn LintRule>, FromContextError> {
        Ok(Box::new(Self))
    }
}

impl RuleCheck for DiseaseConsistencyRule {
    type Data<'a> = (List<'a, Diagnosis>, List<'a, Disease>);

    fn check(&self, data: Self::Data<'_>) -> Vec<LintViolation> {
        let mut violations = vec![];

        let disease_terms: Vec<(&str, &str)> = data
            .1
            .iter()
            .filter_map(|disease| {
                disease
                    .inner
                    .term
                    .as_ref()
                    .map(|oc| (oc.id.as_str(), oc.label.as_str()))
            })
            .collect();

        for diagnosis in data.0.iter() {
            if let Some(oc) = &diagnosis.inner.disease
                && !disease_terms.contains(&(oc.id.as_str(), oc.label.as_str()))
            {
                violations.push(LintViolation::new(
                    ViolationSeverity::Warning,
                    LintRule::rule_id(self),
                    NonEmptyVec::with_single_entry(
                        diagnosis.pointer().clone().down("disease").clone(),
                    ),
                ))
            }
        }

        violations
    }
}

#[register_report(id = "INTER001")]
struct DiseaseConsistencyReport;

impl ReportFromContext for DiseaseConsistencyReport {
    fn from_context(_: &LinterContext) -> Result<Box<dyn RegisterableReport>, FromContextError> {
        Ok(Box::new(Self))
    }
}

impl CompileReport for DiseaseConsistencyReport {
    fn compile_report(&self, full_node: &dyn Node, lint_violation: &LintViolation) -> ReportSpecs {
        let violation_ptr = lint_violation.first_at().clone();
        let mut interpretation_ptr = violation_ptr.clone();

        let interpretation_id = full_node
            .value_at(interpretation_ptr.up().up())
            .expect("Interpretation should have been there")
            .get("id")
            .expect("Interpretation ID should have been there")
            .clone();

        ReportSpecs::from_violation(
             lint_violation,
             format!("Found disease in interpretation {interpretation_id} that is not present in diseases section")
                .to_string(),
             vec![LabelSpecs::new(
                 LabelPriority::Primary,
                 full_node.span_at(&violation_ptr).unwrap().clone(),
                String::default(),
             )],
             vec![],
        )
    }
}

#[register_patch(id = "INTER001")]
struct DiseaseConsistencyPatch;

impl PatchFromContext for DiseaseConsistencyPatch {
    fn from_context(_: &LinterContext) -> Result<Box<dyn RegisterablePatch>, FromContextError> {
        Ok(Box::new(Self))
    }
}

impl CompilePatches for DiseaseConsistencyPatch {
    fn compile_patches(&self, value: &dyn Node, lint_violation: &LintViolation) -> Vec<Patch> {
        let oc: OntologyClass = serde_json::from_value(
            value
                .value_at(lint_violation.first_at())
                .unwrap()
                .as_ref()
                .clone(),
        )
        .unwrap();

        let disease: Value = serde_json::to_value(Disease {
            term: Some(oc),
            ..Default::default()
        })
        .unwrap();

        let instruction = PatchInstruction::Add {
            at: Pointer::at_root().down("diseases").clone(),
            value: Value::Array(vec![disease]),
        };

        vec![Patch::new(NonEmptyVec::with_single_entry(instruction))]
    }
}
