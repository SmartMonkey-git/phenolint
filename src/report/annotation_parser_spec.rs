use annotate_snippets::AnnotationKind;
use std::ops::Range;

struct AnnotationParserSpec {
    kind: AnnotationKind,
    label: String,
    span: Range<usize>,
    highlight_source: bool,
}
