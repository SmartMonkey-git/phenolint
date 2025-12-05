use crate::tree::pointer::Pointer;
use crate::tree::traits::{LocatableNode, RetrievableNode};
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Range;

pub struct DynamicNode {
    pub inner: Value,
    spans: HashMap<Pointer, Range<usize>>,
    pointer: Pointer,
}

impl DynamicNode {
    pub fn new(value: &Value, span: &HashMap<Pointer, Range<usize>>, pointer: Pointer) -> Self {
        DynamicNode {
            inner: value.clone(),
            spans: span.clone(),
            pointer,
        }
    }
}

impl RetrievableNode for DynamicNode {
    fn value_at(&self, ptr: &Pointer) -> Option<Cow<'_, Value>> {
        Some(Cow::Borrowed(self.inner.pointer(ptr.position())?))
    }
}

impl LocatableNode for DynamicNode {
    fn span_at(&self, ptr: &Pointer) -> Option<&Range<usize>> {
        self.spans.get(ptr)
    }

    fn pointer(&self) -> &Pointer {
        &self.pointer
    }
}

pub struct MaterializedNode<T> {
    pub inner: T,
    spans: HashMap<Pointer, Range<usize>>,
    pointer: Pointer,
}

impl<T> MaterializedNode<T> {
    pub fn new(
        materialized_node: T,
        spans: HashMap<Pointer, Range<usize>>,
        pointer: Pointer,
    ) -> Self {
        MaterializedNode {
            inner: materialized_node,
            spans,
            pointer,
        }
    }

    pub(crate) fn from_dynamic(materialized: T, dyn_node: &DynamicNode) -> Self {
        Self::new(
            materialized,
            dyn_node.spans.clone(),
            dyn_node.pointer().clone(),
        )
    }
}

impl<T: Serialize> RetrievableNode for MaterializedNode<T> {
    fn value_at(&self, ptr: &Pointer) -> Option<Cow<'_, Value>> {
        let node_opt = serde_json::to_value(&self.inner).ok()?;
        let value = node_opt.pointer(ptr.position())?.clone();
        Some(Cow::Owned(value))
    }
}

impl<T> LocatableNode for MaterializedNode<T> {
    fn span_at(&self, ptr: &Pointer) -> Option<&Range<usize>> {
        self.spans.get(ptr)
    }

    fn pointer(&self) -> &Pointer {
        &self.pointer
    }
}
