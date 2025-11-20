### How to implement a Rule

A rule consists of three components - the rule check, a visual report and an optional patch. Each of them should be
implemented in their own struct.
Start off with the actual rule.

Add your implementation to: `src/rules`

#### Here is an example that you can copy and adjust:

```rust
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::linter_context::LinterContext;
use crate::rules::traits::{BoxedRuleCheck, RuleCheck, RuleFromContext};
use crate::tree::node::Node;
use crate::tree::pointer::Pointer;
use phenolint_macros::register_rule;
use phenopackets::schema::v2::Phenopacket;

/// # CUST001
/// ## What it does
/// Nothing really. It's here to check if custom implementations work.
///
/// ## Why is this bad?
/// Don't know. Ask Deep Thought.
#[register_rule(id = "CUST001")] // TODO: Change the ID to a unique one.
struct CustomRule;

impl RuleFromContext for CustomRule {
    fn from_context(
        _: &LinterContext,
    ) -> Result<BoxedRuleCheck<Self::CheckType>, FromContextError> { // TODO: Think about if your rule needs any dependencies.
        Ok(Box::new(CustomRule))
    }
}

impl RuleCheck for CustomRule {
    type CheckType = Phenopacket; // <--- TODO: Choose which part of the phenopacket your rule should be linting.

    // Checks whether the part of the phenopacket violates the rule or not. If it does, this should return a LintViolation for each transgression.
    fn check(&self, _: &Self::CheckType, node: &Node) -> Vec<LintViolation> {
        vec![LintViolation::new(
            Self::RULE_ID,
            vec![Pointer::new(node.pointer.clone().down("id").position())],
        )]
    }
}
```

If the violations of your rule can be fixed you should also write a patch. Add the implementation to:
`src/patches/compilers`

```rust
use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::patches::enums::PatchInstruction;
use crate::patches::patch::Patch;
use crate::patches::patch_registration;
use crate::patches::patch_registration::PatchRegistration;
use crate::patches::traits::RulePatch;
use crate::patches::traits::{CompilePatches, PatchFromContext, RegisterablePatch};
use crate::tree::node::Node;
use phenolint_macros::register_patch;

#[register_patch(id = "CUST001")] //TODO: This id needs to align with your rule's id.
struct CustomRulePatchCompiler;

impl PatchFromContext for CustomRulePatchCompiler {
    fn from_context(_: &LinterContext) -> Result<Box<dyn RegisterablePatch>, FromContextError> { //TODO: Think about if you patch needs any dependencies.
        Ok(Box::new(CustomRulePatchCompiler))
    }
}

impl CompilePatches for CustomRulePatchCompiler {
    // This function should return one or multiple patches per violation.
    // It should only return several patches, if there is more than one way to fix the violation and both would lead to a different phenopacket. 
    fn compile_patches(&self, node: &Node, _: &LintViolation) -> Vec<Patch> {
        vec![Patch::new(vec![PatchInstruction::Remove {
            at: node.pointer.clone().down("id").clone(),
        }])]
    }
}
```

Lastly, you need to implement a report that will be printed to the console. Your implementation needs to be added to:
`src/report/compilers`

```rust
#[register_report(id = "CUST001")]  //TODO: This id needs to align with your rule's id.
struct CustomRuleReportCompiler;

impl ReportFromContext for CustomRuleReportCompiler {
    fn from_context(_: &LinterContext) -> Result<Box<dyn RegisterableReport>, FromContextError> { //TODO: Think about if you patch needs any dependencies.
        Ok(Box::new(CustomRuleReportCompiler))
    }
}

impl CompileReport for CustomRuleReportCompiler {
    // You probably want to try out some settings and configurations with this object to find out how your report is printed.
    // You just need to implement the ReportSpecs.
    fn compile_report(&self, node: &Node, violation: &LintViolation) -> ReportSpecs {
        ReportSpecs::new(DiagnosticSpec {
            severity: Severity::Help,
            code: Self::RULE_ID.to_string(),
            message: "This is a custom violation".to_string(),
            labels: vec![LabelSpecs {
                style: LabelStyle::Primary,
                range: node
                    .span(
                        violation
                            .at()
                            .first()
                            .expect("Pointer should have been there."),
                    )
                    .unwrap()
                    .clone(),
                message: "Error was here".to_string(),
            }],
            notes: vec![],
        })
    }
}
```

## Testing

To check if your implementation works add an integration test to `./tests`. There are helpful test utilities:

```rust
mod common;
use crate::common::asserts::LintResultAssertSettings;
use crate::common::construction::minimal_valid_phenopacket;
use crate::common::test_functions::run_rule_test;
use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature};
use rstest::rstest;

#[rstest]
fn test_curie_format_rule() {
    let mut pp = minimal_valid_phenopacket();

    pp.phenotypic_features = vec![
        PhenotypicFeature {
            r#type: Some(OntologyClass {
                id: "invalid_id:31nm".to_string(),
                label: "some pf".to_string(),
            }),
            ..Default::default()
        },
        PhenotypicFeature {
            r#type: Some(OntologyClass {
                id: "HP:0002090".to_string(),
                label: "Pneumonia".to_string(),
            }),
            ..Default::default()
        },
    ];

    let rule_id = "CURIE001";
    let assert_settings = LintResultAssertSettings {
        rule_id,
        n_violations: 1,
        patched_phenopacket: None,
        patches: vec![],
        message_snippets: vec!["invalid_id:31nm", "formatted", "CURIE"],
    };

    run_rule_test(rule_id, &pp, assert_settings);
}

```