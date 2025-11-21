use crate::LinterContext;
use crate::error::FromContextError;
use crate::rules::traits::LintRule;

pub type RuleFactory = fn(context: &LinterContext) -> Rule;
pub type Rule = Result<Box<dyn LintRule>, FromContextError>;

pub struct RuleRegistration {
    pub rule_id: &'static str,
    pub factory: RuleFactory,
}

macro_rules! register_linting_policies {
    () => {
        inventory::collect!(RuleRegistration);

        pub fn all_rule_ids() -> Vec<&'static str> {
            let mut rule_ids = Vec::new();

            fn gather(seen_ids: &mut Vec<&'static str>, type_name: &str)
            where
                RuleRegistration: inventory::Collect,
            {
                for r in inventory::iter::<RuleRegistration>() {
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
