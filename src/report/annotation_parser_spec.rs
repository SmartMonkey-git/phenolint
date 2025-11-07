use annotate_snippets::{Annotation, AnnotationKind};
use std::ops::Range;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AnnotationParserSpec {
    kind: AnnotationKind,
    label: String,
    span: Range<usize>,
    highlight_source: bool,
}

impl AnnotationParserSpec {
    /// Create a new `AnnotationParserSpec` with default values
    pub fn new() -> Self {
        Self {
            kind: AnnotationKind::Primary,
            label: String::new(),
            span: 0..0,
            highlight_source: false,
        }
    }

    pub fn kind(&self) -> &AnnotationKind {
        &self.kind
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn span(&self) -> &Range<usize> {
        &self.span
    }

    pub fn highlight_source(&self) -> bool {
        self.highlight_source
    }

    /// Set the kind
    fn with_kind(mut self, kind: AnnotationKind) -> Self {
        self.kind = kind;
        self
    }

    /// Set the label
    pub fn with_label<S: Into<String>>(mut self, label: S) -> Self {
        self.label = label.into();
        self
    }

    /// Set the span range
    pub fn with_span(mut self, span: Range<usize>) -> Self {
        self.span = span;
        self
    }

    /// Set the highlight_source flag
    pub fn with_highlight_source(mut self, highlight: bool) -> Self {
        self.highlight_source = highlight;
        self
    }

    /// Create a Primary annotation
    pub fn primary() -> Self {
        Self::new().with_kind(AnnotationKind::Primary)
    }

    /// Create a Context annotation
    pub fn context() -> Self {
        Self::new().with_kind(AnnotationKind::Context)
    }

    /// Create a Visible annotation
    pub fn visible() -> Self {
        Self::new().with_kind(AnnotationKind::Visible)
    }

    pub fn annotation(&self) -> Annotation<'static> {
        self.kind()
            .span(self.span().to_owned())
            .label(self.label().to_owned())
            .highlight_source(self.highlight_source())
    }
}
