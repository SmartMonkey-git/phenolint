#![allow(dead_code)]
use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::RuleInitError;
use crate::report::report_registration::ReportRegistration;
use crate::report::specs::ReportSpecs;
use crate::tree::node::Node;
use std::collections::HashMap;

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
    fn compile_report(&self, value: &Node, lint_violation: &LintViolation) -> ReportSpecs;
}

pub trait ReportFromContext {
    fn from_context(context: &LinterContext) -> Result<Box<dyn RegisterableReport>, RuleInitError>;
}

#[derive(Default)]
pub struct ReportRegistry {
    report_compiler: HashMap<String, Box<dyn RegisterableReport>>,
}

impl ReportRegistry {
    pub fn register<R: CompileReport + RuleReport + 'static>(&mut self, rule_id: &str, report: R) {
        self.report_compiler
            .insert(rule_id.to_string(), Box::new(report));
    }

    pub fn get_report_for(
        &self,
        rule_id: &str,
        value: &Node,
        violation: &LintViolation,
    ) -> Option<ReportSpecs> {
        self.report_compiler
            .get(rule_id)
            .map(|report_compiler| report_compiler.compile_report(value, violation))
    }

    pub fn with_all_reports() -> Self {
        let mut registry = Self::default();

        for registration in inventory::iter::<ReportRegistration> {
            (registration.register)(&mut registry);
        }

        registry
    }
}
