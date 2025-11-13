use crate::diagnostics::report::LintReport;
use crate::diagnostics::{LintFinding, LintViolation};
use crate::enums::PatchAction;
use crate::error::{LintResult, RuleInitError};
use crate::json::Pointer;
use crate::linter_context::LinterContext;
use crate::new::json_traverser::Node;
use phenopackets::schema::v2::core::OntologyClass;
use serde_json::Value;

pub trait LintRule: RuleCheck + FromContext {
    const RULE_ID: &'static str;
}

pub trait NodeParser {
    fn parse_ontology_class(value: &Node) -> Option<OntologyClass>
    where
        Self: Sized;
}

pub trait DeserializePhenopackets<T> {
    fn deserialize(bytes: &[u8]) -> T;
}

pub trait CompilePatch<T> {
    fn compile_patch(&self, value: &Node) -> PatchAction;
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

pub trait Lint<T> {
    fn lint(&mut self, input: T, patch: bool, quiet: bool) -> LintResult;
}
