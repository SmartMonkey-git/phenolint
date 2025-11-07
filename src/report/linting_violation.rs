use crate::report::owned_report::OwnedReport;
use annotate_snippets::Renderer;
use annotate_snippets::renderer::DecorStyle;

#[derive(Clone, Debug)]
pub struct LintingViolation {
    rule_id: String,
    report: OwnedReport,
}

impl PartialEq for LintingViolation {
    fn eq(&self, other: &Self) -> bool {
        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        self.rule_id == other.rule_id
            && renderer.render(&[self.report.report()]) == renderer.render(&[other.report.report()])
    }
}
impl LintingViolation {
    pub fn new(rule_id: &str, report: OwnedReport) -> LintingViolation {
        Self {
            rule_id: rule_id.to_string(),
            report,
        }
    }

    pub fn rule_id(&self) -> String {
        self.rule_id.clone()
    }
    pub fn report(&self) -> &OwnedReport {
        &self.report
    }
}
