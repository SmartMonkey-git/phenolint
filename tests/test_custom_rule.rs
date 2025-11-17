use crate::common::construction::linter;
use crate::common::paths::assets_dir;
use phenolint::LinterContext;
use phenolint::diagnostics::LintViolation;
use phenolint::diagnostics::enums::PhenopacketData;
use phenolint::error::FromContextError;
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

mod common;

#[register_rule(id = "CUST001")]
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
fn test_custom_rule(assets_dir: PathBuf) {
    let mut linter = linter(vec!["CUST001"]);

    let test_pp = assets_dir.join("phenopacket.json");

    let pp = fs::read_to_string(test_pp).unwrap();
    let res = linter.lint(pp.as_str(), true, false);

    if let Some(pp) = res.report.patched_phenopacket {
        match pp {
            PhenopacketData::Text(pp_t) => {
                eprintln!("{}", pp_t);
            }
            PhenopacketData::Binary(_) => {}
        }
    } else if let Err(ref e) = res.into_result() {
        eprintln!("{}", e);
        exit(1);
    }
}
