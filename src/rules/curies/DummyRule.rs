use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::RuleInitError;
use crate::register_rule;
use crate::rules::rule_registry::LintingPolicy;
use crate::{FromContext, LintRule, RuleCheck};
use phenolint_macros::register_rule as rr;
use phenopackets::schema::v2::core::PhenotypicFeature;

#[rr(id = "DUMMY001")]
struct DummyRule;

impl FromContext for DummyRule {
    type CheckType = PhenotypicFeature;

    fn from_context(
        context: &LinterContext,
    ) -> Result<Box<dyn RuleCheck<CheckType = Self::CheckType>>, RuleInitError> {
        Ok(Box::new(DummyRule))
    }
}

impl RuleCheck for DummyRule {
    type CheckType = PhenotypicFeature;

    fn check(&self, node: &PhenotypicFeature) -> Vec<LintViolation> {
        println!("{}", Self::RULE_ID);
        println!("{:?}", node);
        vec![]
    }
}
