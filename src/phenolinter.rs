#![allow(dead_code)]
#![allow(unused)]
use crate::config::LinterConfig;
use crate::config::config_loader::ConfigLoader;
use crate::diagnostics::{LintReport, ReportParser};
use crate::error::{InstantiationError, LinterError};
use crate::linter_policy::LinterPolicy;
use crate::patcher::Patcher;
use crate::rules::rule_registry::RuleRegistration;
use crate::traits::{Lint, RuleCheck};
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
}

impl Lint<&str> for Phenolinter {
    fn lint(&mut self, phenostr: &str, patch: bool, quite: bool) -> LintReport {
        // TODO: Understand the conversion here. Why is it lossy, should it be lossy?
        let mut report = self.policy.apply(phenostr.as_ref());

        if !quite {
            for info in &report.findings {
                ReportParser::emit(info.violation().report())
            }
        }

        if patch && report.has_violations() {
            let patched = self.patcher.patch(phenostr, report.patches()).unwrap();
            report.patched_phenopacket = Some(patched)
        }

        report
    }
}

impl Lint<PathBuf> for Phenolinter {
    fn lint(&'_ mut self, phenopath: PathBuf, patch: bool, quite: bool) -> LintReport {
        let phenobytes = std::fs::read(phenopath).expect("Could not read file");
        self.lint(phenobytes.as_slice(), patch, quite)
    }
}

impl Lint<&[u8]> for Phenolinter {
    fn lint(&mut self, phenobytes: &[u8], patch: bool, quite: bool) -> LintReport {
        // TODO: Understand the conversion here. Why is it lossy, should it be lossy?
        let phenostr = String::from_utf8_lossy(phenobytes);
        self.lint(phenostr.as_ref(), patch, quite)
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
        let file_path = tmp_dir.path().join("phenolint.toml");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(TOML_CONFIG).unwrap();

        let linter = Phenolinter::try_from(file_path).expect("Failed to parse phenolint file");
        //assert_eq!(linter.policy.rules.len(), 3);
    }
}
