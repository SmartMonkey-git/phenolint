pub struct RuleRegistration {
    pub rule_id: &'static str,
}

inventory::collect!(RuleRegistration);


#[macro_export]
macro_rules! register_rule {
    ($rule_type:ty) => {
        inventory::submit! {
            RuleRegistration {
                rule_id: <$rule_type>::RULE_ID,
                //factory: || Box::new(<$rule_type>::default()),
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use rstest::rstest;
    use inventory;
    use crate::rules::rule_registry::RuleRegistration;

    #[rstest]
    fn test_rule_id_uniqueness(){
        let mut seen_ids = HashSet::new();
        inventory::iter::<RuleRegistration>().for_each(|r| {
            if seen_ids.contains(&r.rule_id) {
                panic!("rule {} already registered", r.rule_id);
            }
            seen_ids.insert(r.rule_id);
        });
    }
}