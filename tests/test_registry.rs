use phenolint::diagnostics::LintViolation;
use phenolint::error::RuleInitError;
use phenolint::phenolint::Linter;
use phenolint::rules::rule_registry::{BoxedRuleCheck, LintingPolicy};
use phenolint::rules::traits::LintRule;
use phenolint::rules::traits::{RuleCheck, RuleFromContext};
use phenolint::tree::node::Node;
use phenolint::{LinterContext, register_rule};
use phenolint_macros::register_rule as rr;
use phenopackets::schema::v2::core::OntologyClass;
use rstest::rstest;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

#[rr(id = "CURIE002")]
struct SomeRule;

impl RuleFromContext for SomeRule {
    type CheckType = OntologyClass;

    fn from_context(_: &LinterContext) -> Result<BoxedRuleCheck<Self::CheckType>, RuleInitError> {
        Ok(Box::new(SomeRule))
    }
}

impl RuleCheck for SomeRule {
    type CheckType = OntologyClass;

    fn check(&self, parsed_node: &Self::CheckType, _: &Node) -> Vec<LintViolation> {
        println!("Checking node: {:?}", parsed_node);
        println!("Reached: {}", Self::RULE_ID);

        vec![]
    }
}

#[rstest]
fn test() {
    let context = LinterContext::new(
        None,
        vec![
            "CURIE001".to_string(),
            "DUMMY001".to_string(),
            "CURIE002".to_string(),
        ],
    );
    let mut l = Linter::new(context);

    let test_pp = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("phenopacket.json");

    let pp = fs::read(test_pp).unwrap();
    let res = l.lint(pp.as_slice(), false, false);

    if let Err(ref e) = res.into_result() {
        eprintln!("{}", e);
        exit(1);
    }
}
