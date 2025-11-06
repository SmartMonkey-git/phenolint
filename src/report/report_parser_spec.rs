use crate::report::snippet_parser_spec::SnippetParserSpec;
#[allow(dead_code)]
pub enum Level {
    Error,
    Warning,
    Info,
    Note,
    Help,
}
#[allow(dead_code)]
pub struct ReportParserSpec<T> {
    pub level: Level,
    pub primary_title: String,
    pub snippets: Vec<SnippetParserSpec<T>>,
}
