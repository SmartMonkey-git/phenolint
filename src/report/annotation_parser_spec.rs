use annotate_snippets::AnnotationKind;
use std::ops::Range;
#[allow(dead_code)]
struct AnnotationParserSpec {
    kind: AnnotationKind,
    label: String,
    span: Range<usize>,
    highlight_source: bool,
}
