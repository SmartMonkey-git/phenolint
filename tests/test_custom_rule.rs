use crate::common::asserts::LintResultAssertSettings;
use crate::common::assets::json_phenopacket;
use phenolint::patches::patch_registration::PatchRegistration;
use phenolint::report::report_registration::ReportRegistration;
use phenolint::rules::rule_registration::RuleRegistration;
use phenolint::rules::traits::RuleMetaData;

use crate::common::test_functions::run_rule_test;
use phenolint::LinterContext;
use phenolint::diagnostics::LintViolation;
use phenolint::error::FromContextError;
use phenolint::helper::NonEmptyVec;
use phenolint::patches::enums::PatchInstruction;
use phenolint::patches::patch::Patch;
use phenolint::patches::traits::{CompilePatches, PatchFromContext, RegisterablePatch, RulePatch};
use phenolint::report::enums::{LabelPriority, ViolationSeverity};
use phenolint::report::specs::{LabelSpecs, ReportSpecs};
use phenolint::report::traits::{CompileReport, RegisterableReport, ReportFromContext, RuleReport};
use phenolint::rules::traits::LintRule;
use phenolint::rules::traits::{RuleCheck, RuleFromContext};
use phenolint::tree::node_repository::List;
use phenolint::tree::pointer::Pointer;
use phenolint::tree::traits::{Node, UberNode};
use phenolint_macros::{register_patch, register_report, register_rule};
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::OntologyClass;
use rstest::rstest;

mod common;
/// ### CUST001
/// ## What it does
/// Nothing really. It's here to check if custom implementations work.
///
/// ## Why is this bad?
/// Don't know. Ask Deep Thought.
#[register_rule(id = "CUST001")]
struct CustomRule;

impl RuleFromContext for CustomRule {
    fn from_context(_: &LinterContext) -> Result<Box<dyn LintRule>, FromContextError> {
        Ok(Box::new(CustomRule))
    }
}

impl RuleCheck for CustomRule {
    type Data<'a> = List<'a, OntologyClass>;

    fn check(&self, _: Self::Data<'_>) -> Vec<LintViolation> {
        vec![LintViolation::new(
            ViolationSeverity::Info,
            LintRule::rule_id(self),
            NonEmptyVec::with_single_entry(Pointer::at_root().down("id").clone()),
        )]
    }
}

#[register_patch(id = "CUST001")]
struct CustomRulePatchCompiler;

impl PatchFromContext for CustomRulePatchCompiler {
    fn from_context(_: &LinterContext) -> Result<Box<dyn RegisterablePatch>, FromContextError> {
        Ok(Box::new(CustomRulePatchCompiler))
    }
}

impl CompilePatches for CustomRulePatchCompiler {
    fn compile_patches(&self, node: &dyn UberNode, _: &LintViolation) -> Vec<Patch> {
        vec![Patch::new(NonEmptyVec::with_single_entry(
            PatchInstruction::Remove {
                at: node.pointer().clone().down("id").clone(),
            },
        ))]
    }
}

#[register_report(id = "CUST001")]
struct CustomRuleReportCompiler;

impl ReportFromContext for CustomRuleReportCompiler {
    fn from_context(_: &LinterContext) -> Result<Box<dyn RegisterableReport>, FromContextError> {
        Ok(Box::new(CustomRuleReportCompiler))
    }
}

impl CompileReport for CustomRuleReportCompiler {
    fn compile_report(&self, full_node: &dyn UberNode, violation: &LintViolation) -> ReportSpecs {
        let ptr = violation.first_at();

        ReportSpecs::from_violation(
            violation,
            "This is a custom violation".to_string(),
            vec![LabelSpecs::new(
                LabelPriority::Primary,
                full_node
                    .span_at(ptr)
                    .unwrap_or_else(|| panic!("Span should have been at '{}' there", ptr))
                    .clone(),
                "Error was here".to_string(),
            )],
            vec![],
        )
    }
}

#[rstest]
fn test_custom_rule(json_phenopacket: Phenopacket) {
    let settings = LintResultAssertSettings::builder("CUST001")
        .one_violation()
        .patch(Patch::new(NonEmptyVec::with_single_entry(
            PatchInstruction::Remove {
                at: Pointer::new("/id"),
            },
        )))
        .build();

    run_rule_test("CUST001", &json_phenopacket, settings);
}
