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