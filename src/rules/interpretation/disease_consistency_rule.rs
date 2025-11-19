use crate::LinterContext;
use crate::diagnostics::LintViolation;
use std::sync::Arc;
use std::sync::OnceLock;

use crate::error::FromContextError;
use crate::rules::rule_registry::LintingPolicy;
use crate::rules::traits::{BoxedRuleCheck, LintRule, RuleCheck, RuleFromContext};
use crate::tree::node::Node;
use phenolint_macros::register_rule;
use phenopackets::schema::v2::Phenopacket;

#[derive(Debug, Default)]
/// # INTER001
/// ## What it does
/// Checks if all diseases found in the interpretation section are also present in the diseases section.
///
/// ## Why is this bad?
/// It is expected that the disease section contains all diseases associated with a patient.
#[register_rule(id = "INTER001")]
pub struct DiseaseConsistencyRule;

impl RuleFromContext for DiseaseConsistencyRule {
    type CheckType = Phenopacket;

    fn from_context(_: &LinterContext) -> Result<BoxedRuleCheck<Phenopacket>, FromContextError> {
        Ok(Box::new(Self))
    }
}

impl RuleCheck for DiseaseConsistencyRule {
    type CheckType = Phenopacket;

    fn check(&self, _: &Phenopacket, _: &Node) -> Vec<LintViolation> {
        todo!()
    }
}
