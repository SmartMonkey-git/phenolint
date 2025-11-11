use crate::IntoBytes;
use crate::config::LinterConfig;
use crate::config::config_loader::ConfigLoader;
use crate::diagnostics::LintReport;
use crate::error::{InstantiationError, RuleInitError};
use crate::json::PhenopacketCursor;
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
    pub(crate) fn new(rules: Vec<Box<dyn RuleCheck>>) -> LinterPolicy {
        LinterPolicy { rules }
    }
    pub fn apply<T: IntoBytes + Clone>(&self, phenobytes: &T) -> LintReport {
        let mut report = LintReport::default();
        // TODO: Kill expect
        let mut cursor = PhenopacketCursor::new(phenobytes).expect("");
        for rule in &self.rules {
            rule.check(&mut cursor, &mut report)
        }

        report
    }

    pub fn push_rule(&mut self, rule: Box<dyn RuleCheck>) {
        self.rules.push(rule);
    }
    #[allow(dead_code)]
    pub fn try_from_path<P: AsRef<Path>>(path: P) -> Result<Self, InstantiationError> {
        let config: LinterConfig = ConfigLoader::load(PathBuf::from(path.as_ref()))?;
        Ok(LinterPolicy::from(config))
    }
}

impl From<LinterConfig> for LinterPolicy {
    fn from(config: LinterConfig) -> LinterPolicy {
        LinterPolicy::from(config.rule_ids)
    }
}

impl<T, S> From<T> for LinterPolicy
where
    T: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    fn from(rule_ids: T) -> Self {
        let mut policy = LinterPolicy::default();
        let mut seen_rules = HashSet::new();
        let linter_context = LinterContext::default();

        let rule_ids: HashSet<String> = rule_ids
            .into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();

        for r in inventory::iter::<RuleRegistration>() {
            #[allow(clippy::collapsible_if)]
            if rule_ids.contains(r.rule_id) && !seen_rules.contains(&r.rule_id) {
                match (r.factory)(&linter_context) {
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
}
