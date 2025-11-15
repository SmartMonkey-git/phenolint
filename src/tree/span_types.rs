#![allow(dead_code)]
use crate::tree::pointer::Pointer;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Clone)]
pub enum Span {
    Json(JsonSpan),
    Yaml(YamlSpan),
}

impl Span {
    #[allow(unused)]
    pub(crate) fn span(&self, ptr: &Pointer) -> Option<&Range<usize>> {
        match self {
            Span::Json(s) => s.spans.get(ptr),
            Span::Yaml(s) => s.spans.get(ptr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JsonSpan {
    spans: HashMap<Pointer, Range<usize>>,
}

/*impl Clone for JsonSpan {
    fn clone(&self) -> Self {
        if let Some(obj) = self.spans.as_span_object() {
            SpannedValue {
                value: json_spanned_value::Value::Object(),
                start: obj.start,
                end: obj.end,
            };
            JsonSpan::new(SpannedValue::from(obj))
        }
    }
}*/

impl JsonSpan {
    pub fn new(spans: HashMap<Pointer, Range<usize>>) -> JsonSpan {
        JsonSpan { spans }
    }
}
#[derive(Debug, Clone)]
pub struct YamlSpan {
    spans: HashMap<Pointer, Range<usize>>,
}

impl YamlSpan {
    pub fn new(spans: HashMap<Pointer, Range<usize>>) -> YamlSpan {
        YamlSpan { spans }
    }
}
