use crate::json::Pointer;
use crate::new::traits::Spanning;
use spanned_json_parser::SpannedValue;

#[derive(Clone)]
pub enum Span {
    Json(JsonSpan),
    Yaml(YamlSpan),
}

impl Spanning for Span {
    fn span(&self, ptr: &Pointer) -> Option<(usize, usize)> {
        match self {
            Span::Json(s) => Some((1, 2)),
            Span::Yaml(s) => Some((1, 2)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsonSpan {
    spans: SpannedValue,
}

impl JsonSpan {
    pub(crate) fn new(spans: SpannedValue) -> JsonSpan {
        JsonSpan { spans }
    }
}
#[derive(Debug, Clone)]
pub struct YamlSpan {
    spans: Vec<(usize, usize)>,
}

impl YamlSpan {
    pub(crate) fn new(spans: Vec<(usize, usize)>) -> YamlSpan {
        YamlSpan { spans }
    }
}
