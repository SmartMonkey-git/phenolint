#![allow(dead_code)]
#![allow(unused)]
use crate::IntoBytes;
use crate::config::LinterConfig;
use crate::config::config_loader::ConfigLoader;
use crate::diagnostics::{LintReport, ReportParser};
use crate::error::{InstantiationError, LintResult, LinterError, PatchingError};
use crate::linter_policy::LinterPolicy;
use crate::patcher::Patcher;
use crate::rules::rule_registry::RuleRegistration;
use crate::traits::RuleCheck;
use log::warn;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::PhenotypicFeature;
use phenopackets::schema::v2::core::time_element::Element;
use phenopackets::schema::v2::core::{OntologyClass, TimeElement};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;

pub struct Phenolinter {
    policy: LinterPolicy,
    patcher: Patcher,
}

impl Phenolinter {
    pub fn new(policy: LinterPolicy) -> Phenolinter {
        Phenolinter {
            policy,
            patcher: Patcher,
        }
    }

    pub fn lint<T: IntoBytes + Clone>(
        &mut self,
        phenostr: &T,
        patch: bool,
        quite: bool,
    ) -> LintResult {
        let mut report = self.policy.apply(phenostr);

        if !quite {
            for info in report.findings() {
                if let Err(err) = ReportParser::emit(info.violation().report(), phenostr) {
                    warn!(
                        "Unable to parse and emit report for: '{}'",
                        info.violation().rule_id()
                    );
                };
            }
        }

        if patch && report.has_violations() {
            match self.patcher.patch(phenostr, report.patches()) {
                Ok(patched) => report.patched_phenopacket = Some(patched),
                Err(err) => {
                    return LintResult::partial(report, Some(LinterError::PatchingError(err)));
                }
            }
        }

        LintResult::ok(report)
    }
}

impl TryFrom<LinterConfig> for Phenolinter {
    type Error = InstantiationError;

    fn try_from(config: LinterConfig) -> Result<Self, Self::Error> {
        let policy = LinterPolicy::from(config.rule_ids.as_slice());
        Ok(Phenolinter::new(policy))
    }
}

impl TryFrom<PathBuf> for Phenolinter {
    type Error = InstantiationError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        Phenolinter::try_from(&path)
    }
}
impl TryFrom<&PathBuf> for Phenolinter {
    type Error = InstantiationError;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let config: LinterConfig = ConfigLoader::load(path.clone())?;
        Phenolinter::try_from(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_config;
    use ontolius::TermId;
    use rstest::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[fixture]
    fn term_ancestry() -> Vec<TermId> {
        vec![
            "HP:0000448".parse().unwrap(), // scion
            "HP:0005105".parse().unwrap(),
            "HP:0000366".parse().unwrap(),
            "HP:0000271".parse().unwrap(), // progenitor
        ]
    }

    const TOML_CONFIG: &[u8] = br#"
rules = ["CURIE001", "PF006", "INTER001"]
    "#;

    #[rstest]
    fn test_try_from() {
        let tmp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
        let file_path = tmp_dir.path().join(test_config());

        let linter = Phenolinter::try_from(file_path).expect("Failed to parse phenolint file");
    }
}
