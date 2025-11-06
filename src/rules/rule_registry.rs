use crate::linter_context::LinterContext;
use crate::traits::RuleCheck;

pub struct RuleRegistration {
    pub rule_id: &'static str,
    pub factory: fn(context: &LinterContext) -> Option<Box<dyn RuleCheck>>,
}

inventory::collect!(RuleRegistration);

#[macro_export]
macro_rules! register_rule {
    ($rule_type:ty) => {
        inventory::submit! {
            RuleRegistration {
                rule_id: <$rule_type>::RULE_ID,
                factory: |context: &LinterContext| Box::new(<$rule_type>::from_context(context)),
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::rules::rule_registry::RuleRegistration;
    use inventory;
    use regex::Regex;
    use rstest::rstest;
    use std::collections::HashSet;

    #[rstest]
    fn test_rule_id_uniqueness() {
        let mut seen_ids = HashSet::new();
        inventory::iter::<RuleRegistration>().for_each(|r| {
            if seen_ids.contains(&r.rule_id) {
                panic!("rule {} already registered", r.rule_id);
            }
            seen_ids.insert(r.rule_id);
        });
        println!("{:#?}", seen_ids);
    }

    #[rstest]
    fn test_rule_format() {
        let regex = Regex::new("[A-Z]{1,5}[0-9]{3}").unwrap();

        inventory::iter::<RuleRegistration>().for_each(|r| {
            regex.is_match(&r.rule_id);
        });
    }
}
