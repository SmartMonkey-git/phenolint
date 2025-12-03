use crate::report::error::ReportParseError;
use crate::report::specs::ReportSpecs;
use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

#[derive(Default)]
pub struct ReportRenderer;

impl ReportRenderer {
    #[allow(dead_code)]
    pub fn render_into_string(
        report: &ReportSpecs,
        phenostr: &str,
        phenopacket_id: &str,
    ) -> Result<String, ReportParseError> {
        let mut files = SimpleFiles::new();
        let file_id = files.add(phenopacket_id, phenostr);

        let codespan_diagnostic = Self::parse_specs(report, file_id);

        let config = term::Config::default();

        term::emit_into_string(&config, &files, &codespan_diagnostic)
            .map_err(ReportParseError::StringParsing)
    }

    pub fn emit(
        report: &ReportSpecs,
        phenostr: &str,
        phenopacket_id: &str,
    ) -> Result<(), ReportParseError> {
        let mut files = SimpleFiles::new();
        let file_id = files.add(phenopacket_id, phenostr);

        let codespan_diagnostic = Self::parse_specs(report, file_id);

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = term::Config::default();

        term::emit_to_write_style(&mut writer.lock(), &config, &files, &codespan_diagnostic)
            .map_err(ReportParseError::Emit)
    }

    pub fn parse_specs(report_specs: &ReportSpecs, file_id: usize) -> Diagnostic<usize> {
        let mut diagnostic = report_specs.severity().as_codespan_diagnostic();
        diagnostic = diagnostic.with_message(report_specs.message());
        diagnostic = diagnostic.with_code(report_specs.code());

        if !report_specs.labels().is_empty() {
            let labels = report_specs
                .labels()
                .iter()
                .map(|label_spec| {
                    let label = label_spec
                        .style()
                        .as_codespan_label(file_id, label_spec.range());
                    label.with_message(label_spec.message())
                })
                .collect();

            diagnostic = diagnostic.with_labels(labels);
        }

        if !report_specs.notes().is_empty() {
            diagnostic = diagnostic.with_notes(report_specs.notes().to_vec());
        }
        diagnostic
    }
}
