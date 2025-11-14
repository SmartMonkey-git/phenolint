use crate::error::RuleInitError;
use crate::linter_context::LinterContext;
use crate::traits::RuleCheck;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature};

pub type BoxedRuleCheck<N> = Box<dyn RuleCheck<CheckType = N>>;
pub struct LintingPolicy<T> {
    pub rule_id: &'static str,
    pub factory: fn(context: &LinterContext) -> Result<BoxedRuleCheck<T>, RuleInitError>,
}

inventory::collect!(LintingPolicy<OntologyClass>);
inventory::collect!(LintingPolicy<Phenopacket>);
inventory::collect!(LintingPolicy<PhenotypicFeature>);

#[macro_export]
macro_rules! register_rule {
    ($rule_type:ty) => {
        inventory::submit! {

            LintingPolicy::<<$rule_type as RuleCheck>::CheckType> {
                rule_id: <$rule_type>::RULE_ID,
                factory: |context: &LinterContext| <$rule_type>::from_context(context),
            }
        }
    };
}

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
