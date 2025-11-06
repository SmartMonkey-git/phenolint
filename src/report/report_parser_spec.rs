use crate::report::snippet_parser_spec::SnippetParserSpec;

pub enum Level {
    ERROR,
    WARNING,
    INFO,
    NOTE,
    HELP,
}

pub struct ReportParserSpec<T> {
    pub level: Level,
    pub primary_title: String,
    pub snippets: Vec<SnippetParserSpec<T>>,
}
