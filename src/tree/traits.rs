use crate::tree::pointer::Pointer;
use serde_json::Value;
use std::borrow::Cow;
use std::ops::Range;

pub trait Node {
    fn value_at(&'_ self, ptr: &Pointer) -> Option<Cow<'_, Value>>;
    fn span_at(&self, ptr: &Pointer) -> Option<&Range<usize>>;
    fn pointer(&self) -> &Pointer;
}
