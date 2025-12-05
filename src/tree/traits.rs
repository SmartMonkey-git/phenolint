use crate::tree::pointer::Pointer;
use serde_json::Value;
use std::borrow::Cow;
use std::ops::Range;

pub trait Node: LocatableNode + RetrievableNode {}

impl<T: LocatableNode + RetrievableNode> Node for T {}

pub trait LocatableNode {
    fn span_at(&self, ptr: &Pointer) -> Option<&Range<usize>>;
    fn pointer(&self) -> &Pointer;
}

pub trait RetrievableNode {
    fn value_at(&self, ptr: &Pointer) -> Option<Cow<'_, Value>>;
}
