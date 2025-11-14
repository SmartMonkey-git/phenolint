use crate::diagnostics::LintViolation;
use crate::error::{LintResult, RuleInitError};
use crate::linter_context::LinterContext;
use crate::new::node::Node;
use phenopackets::schema::v2::core::OntologyClass;

pub trait LintRule: RuleCheck + FromContext {
    const RULE_ID: &'static str;
}

pub trait FromContext {
    type CheckType;
    fn from_context(
        context: &LinterContext,
    ) -> Result<Box<dyn RuleCheck<CheckType = Self::CheckType>>, RuleInitError>;
}

pub trait RuleCheck {
    type CheckType;
    fn check(&self, node: &Self::CheckType) -> Vec<LintViolation>;
}

pub trait NodeParser {
    fn parse_ontology_class(value: &Node) -> Option<OntologyClass>
    where
        Self: Sized;
}

pub trait Lint<T> {
    fn lint(&mut self, input: T, patch: bool, quiet: bool) -> LintResult;
}
