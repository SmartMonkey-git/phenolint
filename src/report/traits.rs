use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::report::specs::ReportSpecs;
use crate::tree::traits::Node;

pub trait RuleReport: ReportFromContext + RegisterableReport + CompileReport {
    const RULE_ID: &'static str;
}

pub trait RegisterableReport {
    fn compile_report(&self, value: &dyn Node, lint_violation: &LintViolation) -> ReportSpecs;
    fn rule_id(&self) -> String;
}

impl<T: CompileReport + Send + RuleReport> RegisterableReport for T {
    fn compile_report(&self, value: &dyn Node, lint_violation: &LintViolation) -> ReportSpecs {
        CompileReport::compile_report(self, value, lint_violation)
    }

    fn rule_id(&self) -> String {
        Self::RULE_ID.to_string()
    }
}

pub trait CompileReport {
    fn compile_report(&self, full_node: &dyn Node, lint_violation: &LintViolation) -> ReportSpecs;
}

pub trait ReportFromContext {
    fn from_context(
        context: &LinterContext,
    ) -> Result<Box<dyn RegisterableReport>, FromContextError>;
}
