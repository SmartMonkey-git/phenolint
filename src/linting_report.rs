use crate::enums::{FixAction};
use phenopackets::schema::v2::Phenopacket;

#[derive(Clone, Debug, PartialEq)]
pub struct LintingViolation{
    rule_id: String,
    message: String,
}

impl LintingViolation{
    pub fn new(rule_id: &str, message: &str) -> LintingViolation{
        Self{rule_id: rule_id.to_string(), message: message.to_string()}
    }

    pub fn rule_id(&self) -> String { self.rule_id.clone() }
    pub fn message(&self) -> String { self.message.clone() }
}

#[derive(Clone, Debug)]
pub struct LintReportInfo {
    violation: LintingViolation,
    fix: Option<FixAction>,
}

impl LintReportInfo {
    pub fn new(violation: LintingViolation, fix: Option<FixAction>) -> Self {
        Self { violation, fix }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LintReport {
    pub fixed_phenopacket: Option<Phenopacket>,
    pub report_info: Vec<LintReportInfo>,
}

impl LintReport {
    pub fn new() -> LintReport {
        LintReport {
            fixed_phenopacket: None,
            report_info: Vec::new(),
        }
    }

    pub fn violations(&self) -> Vec<LintingViolation> {
        self.report_info
            .iter()
            .map(|i| i.violation.clone())
            .collect()
    }

    pub fn fixes(&self) -> Vec<FixAction> {
        self.report_info.clone().into_iter().filter_map(|ri| ri.fix).collect()
    }

    pub fn push_violation(&mut self, violation: LintingViolation) {
        self.report_info.push(LintReportInfo::new(violation, None));
    }

    pub fn push_info(&mut self, info: LintReportInfo) {
        self.report_info.push(info);
    }

    pub fn has_violations(&self) -> bool {
        !self.report_info.is_empty()
    }

    pub fn has_fixes(&self) -> bool {
        for info in &self.report_info {
            if info.fix.is_some() {
                return true;
            }
        }
        false
    }
}
