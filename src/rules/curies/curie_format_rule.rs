use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::linter_context::LinterContext;
use crate::rules::rule_registry::LintingPolicy;
use crate::rules::traits::{BoxedRuleCheck, LintRule, RuleCheck, RuleFromContext};
use crate::tree::node::Node;
use phenolint_macros::register_rule;
use phenopackets::schema::v2::core::OntologyClass;
use regex::Regex;
use std::sync::Arc;
use std::sync::OnceLock;

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
    fn from_context(_: &LinterContext) -> Result<BoxedRuleCheck<OntologyClass>, FromContextError> {
        Ok(Box::new(CurieFormatRule {
            regex: Regex::new("^[A-Z][A-Z0-9_]+:[A-Za-z0-9_]+$").expect("Invalid regex"),
        }))
    }
}

impl RuleCheck for CurieFormatRule {
    type CheckType = OntologyClass;

    fn check(&self, pared_node: &OntologyClass, node: &Node) -> Vec<LintViolation> {
        let mut violations = vec![];

        if !self.regex.is_match(&pared_node.id) {
            let mut ptr = node.pointer.clone();
            ptr.down("id");

            violations.push(LintViolation::new(Self::RULE_ID, vec![ptr]))
        }
        violations
    }
}
