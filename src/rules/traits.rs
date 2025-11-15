use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::RuleInitError;
use crate::tree::node::Node;

pub trait LintRule: RuleCheck + RuleFromContext {
    const RULE_ID: &'static str;
}

pub trait RuleFromContext {
    type CheckType;
    fn from_context(
        context: &LinterContext,
    ) -> Result<Box<dyn RuleCheck<CheckType = Self::CheckType>>, RuleInitError>;
}

pub trait RuleCheck: Send + Sync {
    type CheckType;
    fn check(&self, parsed_node: &Self::CheckType, node: &Node) -> Vec<LintViolation>;
}

pub type BoxedRuleCheck<N> = Box<dyn RuleCheck<CheckType = N>>;
