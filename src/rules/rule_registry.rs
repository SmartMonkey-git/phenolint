use crate::error::FromContextError;
use crate::linter_context::LinterContext;
use crate::rules::traits::BoxedRuleCheck;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature};
use std::sync::Arc;

pub type RuleFactory<T> = fn(context: &LinterContext) -> RuleCheckResult<T>;
pub type RuleCheckResult<T> = Arc<Result<BoxedRuleCheck<T>, FromContextError>>;

pub struct LintingPolicy<T> {
    pub rule_id: &'static str,
    pub factory: RuleFactory<T>,
}

inventory::collect!(LintingPolicy<OntologyClass>);
inventory::collect!(LintingPolicy<Phenopacket>);
inventory::collect!(LintingPolicy<PhenotypicFeature>);

#[cfg(test)]
mod tests {
    use crate::rules::rule_registry::LintingPolicy;
    use inventory;
    use phenopackets::schema::v2::core::OntologyClass;
    use rstest::rstest;
    use std::collections::HashSet;

    #[rstest]
    fn test_rule_id_uniqueness() {
        let mut seen_ids = HashSet::new();

        inventory::iter::<LintingPolicy<OntologyClass>>().for_each(|r| {
            if seen_ids.contains(&r.rule_id) {
                panic!("rule {} already registered", r.rule_id);
            }
            seen_ids.insert(r.rule_id);
        });
    }

    /*
    #[rstest]
    fn test_rule_format() {
        let regex = Regex::new("[A-Z]{1,5}[0-9]{3}").unwrap();

        inventory::iter::<RuleRegistration>().for_each(|r| {
            regex.is_match(r.rule_id);
        });
    }*/
}
