use crate::config::config_loader::ConfigLoader;
use crate::config::linter_config::LinterConfig;
use crate::error::InstantiationError;
use crate::linting_report::LintReport;
use crate::rules::rule_registry::RuleRegistration;
use crate::traits::RuleCheck;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use crate::linter_context::LinterContext;

#[derive(Default)]
pub struct LintingPolicy {
    rules: Vec<Box<dyn RuleCheck>>,
}

impl LintingPolicy {
    pub(crate) fn new(rules: Vec<Box<dyn RuleCheck>>) -> LintingPolicy {
        LintingPolicy { rules }
    }
    pub fn apply(&self, phenobytes: &[u8]) -> LintReport {
        let mut report = LintReport::default();

        for rule in &self.rules {
            rule.check(phenobytes, &mut report)
        }

        report
    }

    pub fn push_rule(&mut self, rule: Box<dyn RuleCheck>) {
        self.rules.push(rule);
    }

    pub fn try_from_path<P: AsRef<Path>>(path: P) -> Result<Self, InstantiationError> {
        let config: LinterConfig = ConfigLoader::load(PathBuf::from(path.as_ref()))?;
        Ok(LintingPolicy::from(config))
    }
}

impl From<LinterConfig> for LintingPolicy {
    fn from(config: LinterConfig) -> LintingPolicy {
        LintingPolicy::from(config.rule_ids)
    }
}

impl<T, S> From<T> for LintingPolicy
where
    T: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    fn from(rule_ids: T) -> Self {
        let mut policy = LintingPolicy::default();
        let mut seen_rules = HashSet::new();
        let linter_context = LinterContext::default();

        let rule_ids: HashSet<String> = rule_ids
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();

        for r in inventory::iter::<RuleRegistration>() {
            if rule_ids.contains(r.rule_id) && !seen_rules.contains(&r.rule_id) {
                 if let Some(rule)  = (r.factory)(&linter_context){
                policy.push_rule(rule);
                }
            }
            seen_rules.insert(r.rule_id);
        }

        policy
    }
}
