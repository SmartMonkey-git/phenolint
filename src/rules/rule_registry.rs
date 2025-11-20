use crate::error::FromContextError;
use crate::linter_context::LinterContext;
use crate::rules::traits::LintRule;
use std::collections::HashSet;
use std::sync::Arc;

pub type RuleFactory = fn(context: &LinterContext) -> Rule;
pub type Rule = Arc<Result<Box<dyn LintRule>, FromContextError>>;

pub struct LintingPolicy {
    pub rule_id: &'static str,
    pub factory: RuleFactory,
}

macro_rules! register_linting_policies {
    () => {
        inventory::collect!(LintingPolicy);

        pub fn all_rule_ids() -> Vec<&'static str> {
            let mut rule_ids = Vec::new();

            fn gather(seen_ids: &mut Vec<&'static str>, type_name: &str)
            where
                LintingPolicy: inventory::Collect,
            {
                for r in inventory::iter::<LintingPolicy>() {
                    if seen_ids.contains(&r.rule_id) {
                        panic!(
                            "rule {} already registered (found in {})",
                            r.rule_id, type_name
                        );
                    }
                    seen_ids.push(r.rule_id);
                }
            }

            gather(&mut rule_ids, stringify!($type));

            rule_ids
        }
    };
}

register_linting_policies!();

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
    use crate::rules::rule_registry::Arc;
    use crate::rules::rule_registry::FromContextError;
    use crate::rules::rule_registry::LintingPolicy;
    use crate::rules::rule_registry::check_duplicate_rule_ids;
    use crate::rules::traits::LintRule;
    use crate::rules::traits::RuleCheck;
    use crate::rules::traits::{BoxedRuleCheck, RuleFromContext};
    use crate::tree::pointer::Pointer;
    use phenolint_macros::register_rule;
    use rstest::rstest;
    use std::any::Any;
    use std::sync::OnceLock;

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
        fn check(&self) -> Vec<LintViolation> {
            todo!()
        }
    }

    #[rstest]
    #[should_panic(expected = "rule")]
    fn test_rule_id_uniqueness() {
        check_duplicate_rule_ids();
    }
}
