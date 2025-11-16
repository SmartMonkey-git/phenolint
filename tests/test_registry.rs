use phenolint::LinterContext;
use phenolint::diagnostics::LintViolation;
use phenolint::error::FromContextError;
use phenolint::phenolint::Phenolint;
use phenolint::rules::rule_registry::LintingPolicy;
use phenolint::rules::traits::{BoxedRuleCheck, LintRule};
use phenolint::rules::traits::{RuleCheck, RuleFromContext};
use phenolint::traits::Lint;
use phenolint::tree::node::Node;
use phenolint_macros::register_rule;
use phenopackets::schema::v2::core::OntologyClass;
use rstest::rstest;
use std::fs;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use std::sync::OnceLock;

pub fn assets_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("assets")
}

#[register_rule(id = "CURIE002")]
struct SomeRule;

impl RuleFromContext for SomeRule {
    type CheckType = OntologyClass;

    fn from_context(
        _: &LinterContext,
    ) -> Result<BoxedRuleCheck<Self::CheckType>, FromContextError> {
        Ok(Box::new(SomeRule))
    }
}

impl RuleCheck for SomeRule {
    type CheckType = OntologyClass;

    fn check(&self, _: &Self::CheckType, _: &Node) -> Vec<LintViolation> {
        //println!("Checking node: {:?}", parsed_node);
        //println!("Reached: {}", Self::RULE_ID);

        vec![]
    }
}

#[rstest]
fn test() {
    let context = LinterContext::new(None);
    let mut l = Phenolint::new(
        context,
        vec![
            "CURIE001".to_string(),
            "DUMMY001".to_string(),
            "CURIE002".to_string(),
        ],
    );

    let test_pp = assets_dir().join("phenopacket.pb");

    let pp = fs::read(test_pp).unwrap();
    let res = l.lint(pp.as_slice(), false, false);

    if let Err(ref e) = res.into_result() {
        eprintln!("{}", e);
        exit(1);
    }
}
