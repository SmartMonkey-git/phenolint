use crate::diagnostics::report::LintReport;
use crate::error::{LinterError, RuleInitError};
use crate::linter_context::LinterContext;

pub trait LintRule: RuleCheck + FromContext {
    const RULE_ID: &'static str;
}

pub trait FromContext {
    fn from_context(context: &LinterContext) -> Result<Box<dyn RuleCheck>, RuleInitError>;
}

pub trait RuleCheck {
    fn check(&self, phenostr: &str, report: &mut LintReport);
}

pub trait Lint<T> {
    fn lint(&mut self, input: T, patch: bool, quiet: bool) -> Result<LintReport, LinterError>;
}
