#![allow(dead_code)]
#![allow(unused)]
use crate::error::{InstantiationError, LinterError};
use crate::linting_report::LintReport;
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
use crate::config::config_loader::ConfigLoader;
use crate::config::linter_config::LinterConfig;
use crate::rules::rule_registry::RuleRegistration;

struct PhenopacketLinter {
    rules: Vec<Box<dyn RuleCheck>>,
}

impl Lint<Phenopacket> for PhenopacketLinter {
    fn lint(&mut self, phenopacket: Phenopacket, fix: bool) -> LintReport {
        let mut phenopacket = phenopacket.clone();
        let mut report = LintReport::new();

        for rule in &self.rules {
            rule.check(&phenopacket, &mut report);
        }

        if fix && report.has_violations() {
            let fix_res = self.fix(&mut phenopacket, &report);
            report.fixed_phenopacket = Some(phenopacket)
        }

        report
    }
}

impl Lint<PathBuf> for PhenopacketLinter {
    fn lint(&mut self, path: PathBuf, fix: bool) -> LintReport {
        let content = std::fs::read_to_string(path).expect("Failed to read file");
        let mut phenopacket: Phenopacket =
            serde_json::from_str(&content).expect("Failed to parse phenopacket");
        self.lint(phenopacket, fix)
    }
}

impl Lint<&[u8]> for PhenopacketLinter {
    fn lint(&mut self, bytes: &[u8], fix: bool) -> LintReport {
        let mut phenopacket: Phenopacket =
            serde_json::from_slice(bytes).expect("Failed to parse phenopacket");
        self.lint(phenopacket, fix)
    }
}

impl PhenopacketLinter {
    pub fn new(rules: Vec<Box<dyn RuleCheck>>) -> PhenopacketLinter {
        PhenopacketLinter { rules }
    }

    fn fix(&self, phenopacket: &mut Phenopacket, report: &LintReport) -> Result<(), LinterError> {
        let mut seen = HashSet::new();
        phenopacket.phenotypic_features.retain(|feature| {
            if let Some(f) = &feature.r#type {
                seen.insert(f.id.clone())
            } else {
                true
            }
        });

        Ok(())
    }
}


impl TryFrom<LinterConfig> for PhenopacketLinter {
    type Error = InstantiationError;

    fn try_from(config: LinterConfig) -> Result<Self, Self::Error> {

        let mut rules : Vec<Box<dyn RuleCheck>> = Vec::new();
        let mut seen_rules = HashSet::new();
        inventory::iter::<RuleRegistration>().for_each(|r| {
            if config.rule_ids.contains(&r.rule_id.to_string()) && !seen_rules.contains(&r.rule_id) {
                rules.push((r.factory)());
            }
            seen_rules.insert(r.rule_id);
        });
        Ok(PhenopacketLinter::new(rules))
    }
}

impl TryFrom<PathBuf> for PhenopacketLinter
{
    type Error = InstantiationError;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        PhenopacketLinter::try_from(&path)
    }
}
impl TryFrom<&PathBuf> for PhenopacketLinter
{
    type Error = InstantiationError;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let config: LinterConfig = ConfigLoader::load(path.clone())?;
        PhenopacketLinter::try_from(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontolius::TermId;
    use std::fs::File;
    use std::io::Write;
    use rstest::*;
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
    fn test_try_from(){
        let tmp_dir= tempfile::tempdir().expect("Failed to create temporary directory");
        let file_path = tmp_dir.path().join("phenolint.toml");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(TOML_CONFIG).unwrap();

        let linter = PhenopacketLinter::try_from(file_path).expect("Failed to parse phenolint file");
        assert_eq!(linter.rules.len(), 3);
    }
}
