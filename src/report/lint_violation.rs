use crate::report::owned_report::OwnedReport;
use annotate_snippets::Renderer;
use annotate_snippets::renderer::DecorStyle;

#[derive(Clone, Debug)]
pub struct LintViolation {
    rule_id: String,
    report: OwnedReport,
}

impl PartialEq for LintViolation {
    fn eq(&self, other: &Self) -> bool {
        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        self.rule_id == other.rule_id
            && renderer.render(&[self.report.report()]) == renderer.render(&[other.report.report()])
    }
}
impl LintViolation {
    pub fn new(rule_id: &str, report: OwnedReport) -> LintViolation {
        Self {
            rule_id: rule_id.to_string(),
            report,
        }
    }

    pub fn rule_id(&self) -> &str {
        &self.rule_id
    }
    pub fn report(&self) -> &OwnedReport {
        &self.report
    }
}
