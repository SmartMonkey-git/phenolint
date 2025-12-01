use crate::common::asserts::LintResultAssertSettings;
use crate::common::assets::json_phenopacket;
use phenolint::patches::patch_registration::PatchRegistration;
use phenolint::report::report_registration::ReportRegistration;
use phenolint::rules::rule_registration::RuleRegistration;
use phenolint::rules::traits::RuleMetaData;

use crate::common::test_functions::run_rule_test;
use codespan_reporting::diagnostic::{LabelStyle, Severity};
use phenolint::LinterContext;
use phenolint::diagnostics::LintViolation;
use phenolint::error::FromContextError;
use phenolint::patches::enums::PatchInstruction;
use phenolint::patches::patch::Patch;
use phenolint::patches::traits::{CompilePatches, PatchFromContext, RegisterablePatch, RulePatch};
use phenolint::report::specs::{DiagnosticSpec, LabelSpecs, ReportSpecs};
use phenolint::report::traits::{CompileReport, RegisterableReport, ReportFromContext, RuleReport};
use phenolint::rules::traits::LintRule;
use phenolint::rules::traits::{RuleCheck, RuleFromContext};
use phenolint::tree::node_repository::List;
use phenolint::tree::pointer::Pointer;
use phenolint::tree::traits::Node;
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
            LintRule::rule_id(self),
            vec![Pointer::at_root().down("id").clone()],
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
    fn compile_patches(&self, node: &dyn Node, _: &LintViolation) -> Vec<Patch> {
        vec![Patch::new(vec![PatchInstruction::Remove {
            at: node.pointer().clone().down("id").clone(),
        }])]
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
    fn compile_report(&self, full_node: &dyn Node, violation: &LintViolation) -> ReportSpecs {
        let ptr = violation
            .at()
            .first()
            .expect("Pointer should have been there.");

        ReportSpecs::new(DiagnosticSpec {
            severity: Severity::Help,
            code: Self::RULE_ID.to_string(),
            message: "This is a custom violation".to_string(),
            labels: vec![LabelSpecs {
                style: LabelStyle::Primary,
                range: full_node
                    .span_at(ptr)
                    .unwrap_or_else(|| panic!("Span should have been at '{}' there", ptr))
                    .clone(),
                message: "Error was here".to_string(),
            }],
            notes: vec![],
        })
    }
}

#[rstest]
fn test_custom_rule(json_phenopacket: Phenopacket) {
    let settings = LintResultAssertSettings::builder("CUST001")
        .one_violation()
        .patch(Patch::new(vec![PatchInstruction::Remove {
            at: Pointer::new("/id"),
        }]))
        .build();

    run_rule_test("CUST001", &json_phenopacket, settings);
}
