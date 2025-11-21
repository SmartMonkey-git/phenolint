use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::linter_context::LinterContext;
use crate::rules::rule_registration::RuleRegistration;
use crate::rules::traits::{LintData, LintRule, RuleCheck, RuleFromContext};
use crate::tree::pointer::Pointer;
use phenolint_macros::register_rule;
use phenopackets::schema::v2::core::OntologyClass;
use regex::Regex;
use std::any::Any;
use std::collections::HashMap;

/// ### CURIE001
/// ## What it does
/// Identifies CURIE ID that are not formatted correctly.
///
/// ## Why is this bad?
/// Matching incorrectly formatted ID's back to their original sources can cause problems, when
/// computationally using the phenopacket.
#[derive(Debug)]
// #[register_rule(id = "CURIE001",  tagets=[OntologyClass])]
pub struct CurieFormatRule {
    regex: Regex,
    targets: HashMap<Pointer, OntologyClass>,
}

impl RuleFromContext for CurieFormatRule {
    fn from_context(_: &LinterContext) -> Result<Box<dyn LintRule>, FromContextError> {
        Ok(Box::new(CurieFormatRule {
            regex: Regex::new("^[A-Z][A-Z0-9_]+:[A-Za-z0-9_]+$").expect("Invalid regex"),
            targets: HashMap::new(),
        }))
    }
}

impl RuleCheck for CurieFormatRule {
    type Data<'a> = dyn LintData<'a>;

    fn check(&self) -> Vec<LintViolation> {
        let mut violations = vec![];

        for (ptr, oc) in self.targets.iter() {
            if !self.regex.is_match(&oc.id) {
                let mut ptr = ptr.clone();
                ptr.down("id");

                violations.push(LintViolation::new(self.rule_id(), vec![ptr]))
            }
        }

        violations
    }
}
