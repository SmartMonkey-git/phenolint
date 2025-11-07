use crate::report::snippet_parser_spec::SnippetParserSpec;
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub enum Level {
    #[default]
    Error,
    Warning,
    Info,
    Note,
    Help,
}
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct ReportParserSpec<T> {
    level: Level,
    primary_title: String,
    secondary_title: Option<String>,
    snippets: Vec<SnippetParserSpec<T>>,
}

impl<T> ReportParserSpec<T> {
    pub fn new() -> Self {
        Self {
            level: Level::default(),
            primary_title: String::new(),
            secondary_title: None,
            snippets: Vec::new(),
        }
    }

    pub fn get_level(&self) -> &Level {
        &self.level
    }

    pub fn get_primary_title(&self) -> &str {
        &self.primary_title
    }

    pub fn get_snippets(&self) -> &Vec<SnippetParserSpec<T>> {
        &self.snippets
    }

    /// Set the primary title
    pub fn with_primary_title<S: Into<String>>(mut self, title: S) -> Self {
        self.primary_title = title.into();
        self
    }

    pub fn with_secondary_title<S: Into<String>>(mut self, title: Option<S>) -> Self {
        self.secondary_title = title.map(Into::into);
        self
    }

    /// Replace all snippets
    pub fn with_snippets(mut self, snippets: Vec<SnippetParserSpec<T>>) -> Self {
        self.snippets = snippets;
        self
    }

    /// Add a single snippet
    pub fn with_snippet(mut self, snippet: SnippetParserSpec<T>) -> Self {
        self.snippets.push(snippet);
        self
    }

    /// Add multiple snippets
    pub fn with_more_snippets<I>(mut self, snippets: I) -> Self
    where
        I: IntoIterator<Item = SnippetParserSpec<T>>,
    {
        self.snippets.extend(snippets);
        self
    }

    /// Set the level
    fn with_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn error() -> Self {
        Self::new().with_level(Level::Error)
    }

    pub fn warning() -> Self {
        Self::new().with_level(Level::Warning)
    }

    pub fn info() -> Self {
        Self::new().with_level(Level::Info)
    }

    pub fn note() -> Self {
        Self::new().with_level(Level::Note)
    }

    pub fn help() -> Self {
        Self::new().with_level(Level::Help)
    }
}
