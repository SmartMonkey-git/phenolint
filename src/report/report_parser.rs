/*use crate::report::annotation_parser_spec::AnnotationParserSpec;
use crate::report::report_parser_spec::{Level, ReportParserSpec};
use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{
    Annotation, AnnotationKind, Group, Level as SnippetLevel, Patch, Renderer, Report, Snippet,
};

pub struct ParsedGroup {
    pub json: String,
    pub specs: ReportParserSpec<AnnotationParserSpec>,
    pub group: Group<'static>, // or however you can make it work
}

#[allow(dead_code)]
#[derive(Default)]
struct ReportParser;

impl ReportParser {
    pub fn parse(specs: ReportParserSpec<AnnotationParserSpec>, phenobytes: &[u8]) -> ParsedGroup {
        let json = String::from_utf8(phenobytes.to_vec()).unwrap();

        let level = match specs.get_level() {
            Level::Error => SnippetLevel::ERROR,
            Level::Warning => SnippetLevel::WARNING,
            Level::Info => SnippetLevel::INFO,
            Level::Note => SnippetLevel::NOTE,
            Level::Help => SnippetLevel::HELP,
        };

        let snippets: Vec<Snippet<Annotation>> = specs
            .get_snippets()
            .iter()
            .map(|snippet_spec| {
                let mut snip = Snippet::source(json.clone()).fold(snippet_spec.fold);
                if let Some(p) = &snippet_spec.path {
                    snip = snip.path(p);
                }
                if let Some(ln) = snippet_spec.line_start {
                    snip = snip.line_start(ln);
                }
                let markers: &[AnnotationParserSpec] = &snippet_spec.markers;
                let annos: Vec<_> = markers
                    .iter()
                    .map(AnnotationParserSpec::annotation)
                    .collect();

                snip.annotations(annos)
            })
            .collect();

        let group = level
            .primary_title(specs.clone().get_primary_title().to_string())
            .elements(snippets);
        ParsedGroup {
            json,
            specs: specs.clone(),
            group,
        }
    }

    pub fn emit<T>(specs: ReportParserSpec<AnnotationParserSpec>, phenobytes: &[u8]) {
        let report = Self::parse(specs, phenobytes);
        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        anstream::println!("{}", renderer.render(report));
    }
}


EXAMPLE:

        ReportParserSpec::warning()
            .with_primary_title(format!("[{}] CURIE formatted incorrectly", Self::RULE_ID).as_str())
            .with_snippets(vec![SnippetParserSpec::new().with_markers(vec![
                    AnnotationParserSpec::primary()
                        .with_span(curie_start..curie_end)
                        .with_label("Expected CURIE with format CURIE:12345"),
                    AnnotationParserSpec::context()
                        .with_span(context_span_start..context_span_end)
                        .with_label("For this Ontology Class"),
                ])]);

*/

use crate::linting_report::OwnedReport;
use annotate_snippets::Renderer;
use annotate_snippets::renderer::DecorStyle;

#[derive(Default)]
struct ReportParser;

impl ReportParser {
    pub fn parse(report: OwnedReport) -> String {
        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        renderer.render(&[report.report()])
    }

    pub fn emit(report: OwnedReport) {
        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        anstream::println!("{}", renderer.render(&[report.report()]));
    }
}
