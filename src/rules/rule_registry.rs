use crate::linter_context::LinterContext;
use crate::rules::rule_registration::{RuleRegistration, all_rule_ids};
use crate::rules::traits::LintRule;
use log::warn;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct RuleRegistry {
    rules: HashMap<String, Box<dyn LintRule>>,
}

impl RuleRegistry {
    pub fn get(&self, rule_id: &str) -> Option<&dyn LintRule> {
        self.rules.get(rule_id).map(|rule| &**rule)
    }

    pub fn get_mut(&mut self, rule_id: &str) -> Option<&mut Box<dyn LintRule>> {
        self.rules.get_mut(rule_id)
    }

    pub fn with_enabled_rules(enabled_rules: &[String], context: &LinterContext) -> Self {
        let mut registry = HashMap::new();

        for registration in inventory::iter::<RuleRegistration> {
            if enabled_rules
                .iter()
                .any(|r_id| r_id == registration.rule_id)
            {
                match (registration.factory)(context) {
                    Ok(rule) => {
                        registry.insert(registration.rule_id.to_string(), rule);
                    }
                    Err(err) => warn!("Failed to register patch: {}", err),
                }
            }
        }

        Self { rules: registry }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Box<dyn LintRule>)> {
        self.rules.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut Box<dyn LintRule>)> {
        self.rules.iter_mut()
    }

    pub fn rules(&self) -> impl Iterator<Item = &Box<dyn LintRule>> {
        self.rules.values()
    }

    pub fn rules_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn LintRule>> {
        self.rules.values_mut()
    }
}

pub(crate) fn check_duplicate_rule_ids() {
    let all_rule_ids = all_rule_ids();

    let mut seen_rule_ids = HashSet::new();

    for rule_id in all_rule_ids {
        if seen_rule_ids.contains(&rule_id) {
            panic!("rule '{rule_id}' registered twice");
        }
        seen_rule_ids.insert(rule_id);
    }
}

#[cfg(test)]
mod tests {
    use crate::LinterContext;
    use crate::diagnostics::LintViolation;
    use crate::error::FromContextError;
    use crate::rules::curies::curie_format_rule::__LINKER_ERROR_MISSING_REPORT_STRUCT_FOR_CURIE001;
    use crate::rules::rule_registration::RuleRegistration;
    use crate::rules::rule_registry::check_duplicate_rule_ids;
    use crate::rules::traits::LintRule;
    use crate::rules::traits::RuleCheck;
    use crate::rules::traits::RuleFromContext;
    use crate::rules::traits::RuleMetaData;
    use crate::tree::node_repository::List;
    use phenolint_macros::register_rule;
    use phenopackets::schema::v2::core::OntologyClass;
    use rstest::rstest;

    /// ### CURIE001
    /// ## What it does
    /// Is here to trigger the panic.
    ///
    /// ## Why is this bad?
    /// Because having duplicate rule ID's will lead to confusion.
    #[register_rule(id = "CURIE001")]
    struct TestRule;

    impl RuleFromContext for TestRule {
        fn from_context(_: &LinterContext) -> Result<Box<dyn LintRule>, FromContextError> {
            todo!()
        }
    }
    impl RuleCheck for TestRule {
        type Data<'a> = List<'a, OntologyClass>;

        fn check(&self, _: Self::Data<'_>) -> Vec<LintViolation> {
            todo!()
        }
    }

    #[rstest]
    #[should_panic(expected = "rule")]
    fn test_rule_id_uniqueness() {
        check_duplicate_rule_ids();
    }
}
