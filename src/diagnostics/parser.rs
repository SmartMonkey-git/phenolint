use crate::diagnostics::error::ReportParseError;
use crate::diagnostics::specs::ReportSpecs;
use codespan_reporting::diagnostic::{Diagnostic, Label, LabelStyle, Severity};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

#[derive(Default)]
pub struct ReportParser;

impl ReportParser {
    #[allow(dead_code)]
    pub fn parse(report: &ReportSpecs, phenostr: &str) -> Result<String, ReportParseError> {
        let mut files = SimpleFiles::new();
        let file_id = files.add(1, phenostr);

        let codespan_diagnostic = Self::inner_parse(report, file_id);

        let config = term::Config::default();

        term::emit_into_string(&config, &files, &codespan_diagnostic)
            .map_err(ReportParseError::StringParsing)
    }

    pub fn emit(report: &ReportSpecs, phenostr: &str) -> Result<(), ReportParseError> {
        let mut files = SimpleFiles::new();
        let file_id = files.add(1, phenostr);

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

        if let Some(code) = spec.code() {
            diagnostic = diagnostic.with_code(code);
        }

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
