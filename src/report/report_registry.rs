#![allow(dead_code)]
use crate::diagnostics::LintViolation;
use crate::report::report_registration::ReportRegistration;
use crate::report::specs::ReportSpecs;
use crate::report::traits::{CompileReport, RegisterableReport, RuleReport};
use crate::tree::node::Node;
use std::collections::HashMap;

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

    pub fn with_enabled_reports(enabled_rules: &[String]) -> Self {
        let mut registry = Self::default();

        for registration in inventory::iter::<ReportRegistration> {
            if enabled_rules
                .iter()
                .any(|r_id| r_id == registration.rule_id)
            {
                (registration.register)(&mut registry);
            }
        }

        registry
    }
}
