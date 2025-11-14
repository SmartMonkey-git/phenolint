#![allow(unused)]
use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::RuleInitError;
use crate::report::specs::ReportSpecs;
use crate::tree::node::Node;

pub trait RuleReport: ReportFromContext + RegisterableReport + CompileReport {
    const RULE_ID: &'static str;
}

pub trait RegisterableReport {
    fn compile_report(&self, value: &Node, lint_violation: &LintViolation) -> ReportSpecs;
    fn rule_id(&self) -> String;
}

impl<T: CompileReport + Send + RuleReport> RegisterableReport for T {
    fn compile_report(&self, value: &Node, lint_violation: &LintViolation) -> ReportSpecs {
        CompileReport::compile_report(self, value, lint_violation)
    }

    fn rule_id(&self) -> String {
        Self::RULE_ID.to_string()
    }
}

pub trait CompileReport {
    fn compile_report(&self, node: &Node, lint_violation: &LintViolation) -> ReportSpecs;
}

pub trait ReportFromContext {
    fn from_context(context: &LinterContext) -> Result<Box<dyn RegisterableReport>, RuleInitError>;
}
