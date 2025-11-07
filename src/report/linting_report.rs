use crate::enums::Patch;
use crate::report::linting_report_info::LintReportInfo;
use crate::report::linting_violation::LintingViolation;

#[derive(Clone, Debug, Default)]
pub struct LintReport {
    pub patched_phenopacket: Option<String>,
    pub report_info: Vec<LintReportInfo>,
}

impl LintReport {
    pub fn new() -> LintReport {
        LintReport {
            patched_phenopacket: None,
            report_info: Vec::new(),
        }
    }

    pub fn violations(&'_ self) -> Vec<LintingViolation> {
        self.report_info
            .iter()
            .map(|i| i.violation().clone())
            .collect()
    }

    pub fn patches(&self) -> Vec<Patch> {
        self.report_info
            .clone()
            .iter()
            .filter_map(|ri| ri.patch().map_or(None, |p| Some(p.clone())))
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
            if info.patch().is_some() {
                return true;
            }
        }
        false
    }
}
