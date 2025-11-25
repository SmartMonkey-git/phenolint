use crate::report::error::ReportParseError;
use crate::report::specs::ReportSpecs;
use codespan_reporting::diagnostic::{Diagnostic, Label, LabelStyle, Severity};
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

        let codespan_diagnostic = Self::inner_parse(report, file_id);

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

        let codespan_diagnostic = Self::inner_parse(report, file_id);

        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = term::Config::default();

        term::emit_to_write_style(&mut writer.lock(), &config, &files, &codespan_diagnostic)
            .map_err(ReportParseError::Emit)
    }

    pub fn inner_parse(report: &ReportSpecs, file_id: usize) -> Diagnostic<usize> {
        let spec = report.diagnostics();

        let mut diagnostic = match spec.severity {
            Severity::Error => Diagnostic::error(),
            Severity::Warning => Diagnostic::warning(),
            Severity::Help => Diagnostic::help(),
            Severity::Note => Diagnostic::note(),
            Severity::Bug => Diagnostic::bug(),
        };

        diagnostic = diagnostic.with_message(spec.message());

        diagnostic = diagnostic.with_code(&spec.code);

        if !spec.labels.is_empty() {
            let labels = spec
                .labels()
                .iter()
                .map(|label_spec| {
                    let label = match label_spec.style {
                        LabelStyle::Primary => Label::primary(file_id, label_spec.range().clone()),
                        LabelStyle::Secondary => {
                            Label::secondary(file_id, label_spec.range().clone())
                        }
                    };
                    label.with_message(label_spec.message())
                })
                .collect();

            diagnostic = diagnostic.with_labels(labels);
        }

        if !spec.notes.is_empty() {
            diagnostic = diagnostic.with_notes(spec.notes().to_vec());
        }
        diagnostic
    }
}
