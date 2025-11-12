use crate::diagnostics::report::LintReport;
use crate::error::{LintResult, RuleInitError};
use crate::json::Pointer;
use crate::linter_context::LinterContext;
use crate::new::json_traverser::BoxedNode;
use phenopackets::schema::v2::core::OntologyClass;

pub trait LintRule: RuleCheck + FromContext {
    const RULE_ID: &'static str;
}

pub trait PhenopacketNodeTraversal<T> {
    fn traverse(&self) -> Box<dyn Iterator<Item = Box<dyn Node<T>>> + '_>;
}

pub trait Node<T> {
    fn value(&self) -> T;

    fn span(&self) -> Option<(usize, usize)>;
    fn pointer(&self) -> Pointer;
}

pub trait NodeParser<T> {
    fn parse_ontology_class(value: &BoxedNode<T>) -> Option<OntologyClass>
    where
        Self: Sized;
}

pub trait FromContext {
    type CheckType;
    fn from_context(
        context: &LinterContext,
    ) -> Result<Box<dyn RuleCheck<T = Self::CheckType>>, RuleInitError>;
}

pub trait RuleCheck {
    type T;
    fn check(&self, phenostr: &Self::T, report: &mut LintReport);
}

pub trait Lint<T> {
    fn lint(&mut self, input: T, patch: bool, quiet: bool) -> LintResult;
}
