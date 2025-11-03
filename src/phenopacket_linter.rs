#![allow(dead_code)]
#![allow(unused)]
use crate::error::{InstantiationError, LintingError};
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

    fn fix(&self, phenopacket: &mut Phenopacket, report: &LintReport) -> Result<(), LintingError> {
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
        config.
    }
}
impl TryFrom<&PathBuf> for PhenopacketLinter
{
    type Error = InstantiationError;

    fn try_from(value: &PathBuf) -> Result<Self, Self::Error> {
        let config: LinterConfig = ConfigLoader::load(value.clone())?;


    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontolius::TermId;

    use rstest::*;

    #[fixture]
    fn term_ancestry() -> Vec<TermId> {
        vec![
            "HP:0000448".parse().unwrap(), // scion
            "HP:0005105".parse().unwrap(),
            "HP:0000366".parse().unwrap(),
            "HP:0000271".parse().unwrap(), // progenitor
        ]
    }
}
