use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::rules::rule_registration::RuleRegistration;
use crate::tree::pointer::Pointer;
use std::any::Any;
use std::sync::Arc;
use std::sync::OnceLock;

use crate::error::FromContextError;
use crate::rules::traits::{LintRule, RuleCheck, RuleFromContext};
use phenolint_macros::register_rule;

#[derive(Debug, Default)]
/// ### INTER001
/// ## What it does
/// Checks if all diseases found in the interpretation section are also present in the diseases section.
///
/// ## Why is this bad?
/// It is expected that the disease section contains all diseases associated with a patient.
#[register_rule(id = "INTER001")]
pub struct DiseaseConsistencyRule;

impl RuleFromContext for DiseaseConsistencyRule {
    fn from_context(_: &LinterContext) -> Result<Box<dyn LintRule>, FromContextError> {
        Ok(Box::new(Self))
    }
}

impl RuleCheck for DiseaseConsistencyRule {
    fn check(&self) -> Vec<LintViolation> {
        todo!()
    }
}
