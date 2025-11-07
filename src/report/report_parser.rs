use crate::report::owned_report::OwnedReport;
use annotate_snippets::Renderer;
use annotate_snippets::renderer::DecorStyle;

#[derive(Default)]
pub struct ReportParser;

impl ReportParser {
    #[allow(dead_code)]
    pub fn parse(report: &OwnedReport) -> String {
        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        renderer.render(&[report.report()])
    }

    pub fn emit(report: &OwnedReport) {
        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        anstream::println!("{}", renderer.render(&[report.report()]));
    }
}
