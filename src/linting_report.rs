use crate::enums::Patch;
use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{Renderer, Report};

#[derive(Clone, Debug)]
pub struct LintingViolation {
    rule_id: String,
    report: Report<'static>,
}

impl PartialEq for LintingViolation {
    fn eq(&self, other: &Self) -> bool {
        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        self.rule_id == other.rule_id
            && renderer.render(self.report) == renderer.render(other.report)
    }
}
impl LintingViolation {
    pub fn new(rule_id: &str, report: Report<'static>) -> LintingViolation {
        Self {
            rule_id: rule_id.to_string(),
            report,
        }
    }

    pub fn rule_id(&self) -> String {
        self.rule_id.clone()
    }
    pub fn report(&'_ self) -> Report<'_> {
        self.report
    }
}

#[derive(Clone, Debug)]
pub struct LintReportInfo {
    violation: LintingViolation,
    patch: Option<Patch>,
}

impl LintReportInfo {
    pub fn new(violation: LintingViolation, patch: Option<Patch>) -> Self {
        Self { violation, patch }
    }
}

#[derive(Clone, Debug, Default)]
pub struct LintReport {
    pub fixed_phenopacket: Option<Vec<u8>>,
    pub report_info: Vec<LintReportInfo>,
}

impl LintReport {
    pub fn new() -> LintReport {
        LintReport {
            fixed_phenopacket: None,
            report_info: Vec::new(),
        }
    }

    pub fn violations(&'_ self) -> Vec<LintingViolation> {
        self.report_info
            .iter()
            .map(|i| i.violation.clone())
            .collect()
    }

    pub fn fixes(&self) -> Vec<Patch> {
        self.report_info
            .clone()
            .into_iter()
            .filter_map(|ri| ri.patch)
            .collect()
    }

    pub fn push_info(&mut self, info: LintReportInfo) {
        self.report_info.push(info);
    }

    pub fn has_violations(&self) -> bool {
        !self.report_info.is_empty()
    }

    pub fn has_fixes(&self) -> bool {
        for info in &self.report_info {
            if info.patch.is_some() {
                return true;
            }
        }
        false
    }
}
