use crate::linter_context::LinterContext;
use crate::linting_report::LintReport;

pub trait LintRule: RuleCheck + FromContext {
    const RULE_ID: &'static str;
}

pub trait FromContext {
    fn from_context(context: &LinterContext) -> Option<Box<dyn RuleCheck>>;
}

pub trait RuleCheck {
    fn check(&self, phenobytes: &[u8], report: &mut LintReport);
}

pub(crate) trait Lint<T> {
    fn lint(&mut self, input: T, fix: bool) -> LintReport;
}
