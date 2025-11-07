use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct SnippetParserSpec<T> {
    pub path: Option<String>,
    pub line_start: Option<usize>,
    pub markers: Vec<T>,
    pub fold: bool,
}

impl<T> SnippetParserSpec<T> {
    pub fn new() -> Self {
        Self {
            path: None,
            line_start: Some(0),
            markers: Vec::new(),
            fold: false,
        }
    }

    pub fn with_path<S: Into<String>>(mut self, path: Option<S>) -> Self {
        self.path = path.map(Into::into);
        self
    }

    /// Set the starting line number
    pub fn with_line_start(mut self, line_start: Option<usize>) -> Self {
        self.line_start = line_start;
        self
    }

    /// Set markers (replaces existing)
    pub fn with_markers(mut self, markers: Vec<T>) -> Self {
        self.markers = markers;
        self
    }

    /// Add a single marker
    pub fn with_marker(mut self, marker: T) -> Self {
        self.markers.push(marker);
        self
    }

    /// Add multiple markers
    pub fn with_more_markers<I>(mut self, markers: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        self.markers.extend(markers);
        self
    }

    /// Set fold state
    pub fn with_fold(mut self, fold: bool) -> Self {
        self.fold = fold;
        self
    }

    /// Convenience constructors
    pub fn folded() -> Self {
        Self::new().with_fold(true)
    }

    pub fn unfolded() -> Self {
        Self::new().with_fold(false)
    }
}
