use crate::common::paths::{assets_dir, hpo_dir};
use phenolint::LinterContext;
use phenolint::phenolint::Phenolint;

pub fn linter(rules: Vec<&str>) -> Phenolint {
    let context = LinterContext::new(Some(hpo_dir(assets_dir())));
    let rules: Vec<String> = rules.into_iter().map(|s| s.to_string()).collect();
    Phenolint::new(context, rules)
}
