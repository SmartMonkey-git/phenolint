use crate::tree::pointer::Pointer;
use serde_json::Value;
use std::borrow::Cow;
use std::ops::Range;

pub(crate) trait Node {
    fn get_value(&self) -> Cow<Value>;

    fn get_value_at(&self, ptr: &Pointer) -> Option<Cow<Value>>;
    fn get_span(&self, ptr: &Pointer) -> Option<&Range<usize>>;

    fn pointer(&self) -> &Pointer;
}
