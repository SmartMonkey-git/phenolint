use crate::traits::{LintRule, RuleCheck};
use std::sync::Arc;
use ontolius::ontology::csr::FullCsrOntology;
use ontolius::TermId;
use phenopackets::schema::v2::Phenopacket;
use phenolint_macros::lint_rule;
use crate::linting_report::LintReport;

pub mod enums;
pub mod error;
pub mod linting_report;
pub mod phenopacket_linter;
pub mod rules;
#[cfg(test)]
pub(crate) mod test_utils;
pub mod traits;
mod config;

