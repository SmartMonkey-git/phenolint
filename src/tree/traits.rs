use crate::tree::pointer::Pointer;
use serde_json::Value;
use std::borrow::Cow;
use std::ops::Range;

pub trait Node {
    type Output: Clone;

    fn get_node(&'_ self) -> &Self::Output;

    fn get_value_at(&'_ self, ptr: &Pointer) -> Option<Cow<'_, Value>>;
    fn get_span(&self, ptr: &Pointer) -> Option<&Range<usize>>;

    fn pointer(&self) -> &Pointer;
}
