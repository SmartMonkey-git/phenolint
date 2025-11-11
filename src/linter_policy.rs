use crate::config::LinterConfig;
use crate::config::config_loader::ConfigLoader;
use crate::diagnostics::LintReport;
use crate::error::{InstantiationError, RuleInitError};
use crate::linter_context::LinterContext;
use crate::rules::rule_registry::RuleRegistration;
use crate::traits::RuleCheck;
use log::warn;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub struct LinterPolicy {
    rules: Vec<Box<dyn RuleCheck>>,
}

impl LinterPolicy {
    #[allow(dead_code)]
    pub fn new(rules: Vec<Box<dyn RuleCheck>>) -> LinterPolicy {
        LinterPolicy { rules }
    }

    pub fn from_str<T, S>(rule_ids: T, context: &mut LinterContext) -> Self
    where
        T: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut policy = LinterPolicy::default();
        let mut seen_rules = HashSet::new();

        let rule_ids: HashSet<String> = rule_ids
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();

        for r in inventory::iter::<RuleRegistration>() {
            #[allow(clippy::collapsible_if)]
            if rule_ids.contains(r.rule_id) && !seen_rules.contains(&r.rule_id) {
                println!("Rule {}", r.rule_id);
                match (r.factory)(context) {
                    Ok(rule) => {
                        policy.push_rule(rule);
                    }
                    Err(err) => match err {
                        RuleInitError::NeedsHPO => {
                            warn!(
                                "Rule '{}' needs the HPO. HPO not found or not configured",
                                r.rule_id
                            );
                        }
                    },
                }
            }
            seen_rules.insert(r.rule_id);
        }

        policy
    }
    pub fn apply(&self, phenobytes: &str) -> LintReport {
        let mut report = LintReport::default();

        for rule in &self.rules {
            rule.check(phenobytes, &mut report)
        }

        report
    }

    pub fn push_rule(&mut self, rule: Box<dyn RuleCheck>) {
        self.rules.push(rule);
    }
}
