#![allow(unused)]
use crate::LinterContext;
use crate::diagnostics::LintViolation;

use crate::error::RuleInitError;
use crate::rules::rule_registry::{BoxedRuleCheck, LintingPolicy};
use crate::rules::traits::{LintRule, RuleCheck, RuleFromContext};
use crate::tree::node::Node;
use phenolint_macros::register_rule;
use phenopackets::schema::v2::Phenopacket;

#[derive(Debug, Default)]
/// Validates that diseases in interpretations are also present in the diseases list.
///
/// This rule implements the linting check `INTER001`, which ensures consistency between
/// the diseases mentioned in interpretations and those listed in the top-level diseases
/// field of a phenopacket. When a disease is diagnosed in an interpretation, it should
/// also appear in the phenopacket's diseases list for proper data consistency and
/// completeness.
///
/// # Rule Logic
///
/// 1. Extracts all disease terms from interpretation diagnoses
/// 2. Extracts all disease terms from the top-level diseases field
/// 3. Identifies diseases that appear in interpretations but not in the diseases list
/// 4. Reports a `DiseaseConsistency` violation for each missing disease
/// 5. Suggests a `Duplicate` fix action to copy the disease to the diseases list
///
/// # Example
///
/// If an interpretation contains a diagnosis of "Marfan syndrome" (OMIM:154700) but
/// this disease does not appear in the phenopacket's diseases field, the rule will
/// flag this inconsistency. The disease should be added to both locations to maintain
/// data integrity across the phenopacket structure.
#[register_rule(id = "INTER001")]
pub struct DiseaseConsistencyRule;

impl RuleFromContext for DiseaseConsistencyRule {
    type CheckType = Phenopacket;

    fn from_context(_: &LinterContext) -> Result<BoxedRuleCheck<Phenopacket>, RuleInitError> {
        Ok(Box::new(Self))
    }
}

impl RuleCheck for DiseaseConsistencyRule {
    type CheckType = Phenopacket;

    fn check(&self, parsed_node: &Phenopacket, node: &Node) -> Vec<LintViolation> {
        todo!()
    }
}
