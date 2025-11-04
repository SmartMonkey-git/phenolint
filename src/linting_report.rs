use annotate_snippets::{Renderer, Report};
use annotate_snippets::renderer::DecorStyle;
use crate::enums::{FixAction};
use phenopackets::schema::v2::Phenopacket;

#[derive(Clone, Debug)]
pub struct LintingViolation<'a>{
    rule_id: String,
    report: Report<'a>,
}

impl<'a> PartialEq for LintingViolation<'a> {
    fn eq(&self, other: &Self) -> bool {
        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        self.rule_id == other.rule_id && renderer.render(self.report) == renderer.render(other.report)
    }
}
impl<'a> LintingViolation<'a>{
    pub fn new(rule_id: &str, report: Report<'a>) -> LintingViolation<'a>{
        Self{rule_id: rule_id.to_string(),  report }
    }

    pub fn rule_id(&self) -> String { self.rule_id.clone() }
    pub fn report(&self) -> Report<'a> { self.report.clone() }

}

#[derive(Clone, Debug)]
pub struct LintReportInfo<'a> {
    violation: LintingViolation<'a>,
    fix: Option<FixAction>,
}

impl<'a> LintReportInfo<'a> {
    pub fn new(violation: LintingViolation<'a>, fix: Option<FixAction>) -> Self {
        Self { violation, fix }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LintReport<'a> {
    pub fixed_phenopacket: Option<Phenopacket>,
    pub report_info: Vec<LintReportInfo<'a>>,
}

impl<'a> LintReport<'a> {
    pub fn new() -> LintReport<'a> {
        LintReport {
            fixed_phenopacket: None,
            report_info: Vec::new(),
        }
    }

    pub fn violations(&'_ self) -> Vec<LintingViolation<'_>> {
        self.report_info
            .iter()
            .map(|i| i.violation.clone())
            .collect()
    }

    pub fn fixes(&self) -> Vec<FixAction> {
        self.report_info.clone().into_iter().filter_map(|ri| ri.fix).collect()
    }


    pub fn push_info(&mut self, info: LintReportInfo<'a>) {
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
