use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::RuleInitError;

pub trait LintRule: RuleCheck + RuleFromContext {
    const RULE_ID: &'static str;
}

pub trait RuleFromContext {
    type CheckType;
    fn from_context(
        context: &LinterContext,
    ) -> Result<Box<dyn RuleCheck<CheckType = Self::CheckType>>, RuleInitError>;
}

pub trait RuleCheck {
    type CheckType;
    fn check(&self, node: &Self::CheckType) -> Vec<LintViolation>;
}
