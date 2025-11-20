use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::tree::pointer::Pointer;
use std::any::Any;
pub trait LintRule: RuleCheck + RuleFromContext + Send + Sync {
    fn rule_id(&self) -> &str;

    // Boilerplate to allow downcasting from dyn LintRule to ConcreteType
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn supply_node_any(&mut self, _node: &dyn Any, _pointer: &Pointer);
}
pub trait SupplyRule<N> {
    fn supply_rule(&mut self, pointer: &Pointer, node: &N);
}

pub trait RuleFromContext {
    fn from_context(context: &LinterContext) -> Result<Box<dyn LintRule>, FromContextError>
    where
        Self: Sized;
}

pub trait RuleCheck: Send + Sync {
    fn check(&self) -> Vec<LintViolation>;
}

pub type BoxedRuleCheck = Box<dyn RuleCheck>;
