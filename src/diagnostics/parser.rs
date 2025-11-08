use crate::diagnostics::owned_report::OwnedReport;
use annotate_snippets::Renderer;
use annotate_snippets::renderer::DecorStyle;
use ariadne::sources;
use std::fs;

#[derive(Default)]
pub struct ReportParser;

impl ReportParser {
    #[allow(dead_code)]
    pub fn parse(report: &OwnedReport, phenostr: &str) -> String {
        let cache = sources(vec![("stdin", phenostr.to_string())]);

        let mut buffer = Vec::new();
        report.report().write(cache, &mut buffer).unwrap();
        String::from_utf8_lossy(&buffer).to_string()
    }

    pub fn emit(report: &OwnedReport, phenostr: &str) {
        let cache = sources(vec![("stdin", phenostr.to_string())]);
        report.report().eprint(cache).unwrap();
    }
}
