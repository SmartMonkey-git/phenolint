use crate::tree::pointer::Pointer;
use crate::tree::traits::{IndexNode, Node};
use serde::{Deserialize, Serialize};
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

impl IndexNode for DynamicNode {
    fn value_at(&'_ self, ptr: &Pointer) -> Option<Cow<'_, Value>> {
        Some(Cow::Borrowed(self.inner.pointer(ptr.position())?))
    }
}

impl Node for DynamicNode {
    fn span_at(&self, ptr: &Pointer) -> Option<&Range<usize>> {
        self.spans.get(ptr)
    }

    fn pointer(&self) -> &Pointer {
        &self.pointer
    }
}

#[derive(Serialize, Deserialize, Debug)]
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

impl<T: Serialize> IndexNode for MaterializedNode<T> {
    fn value_at(&'_ self, ptr: &Pointer) -> Option<Cow<'_, Value>> {
        let node_opt = serde_json::to_value(&self.inner).ok()?;
        let value = node_opt.pointer(ptr.position())?.clone();
        Some(Cow::Owned(value))
    }
}

impl<T> Node for MaterializedNode<T> {
    fn span_at(&self, ptr: &Pointer) -> Option<&Range<usize>> {
        self.spans.get(ptr)
    }

    fn pointer(&self) -> &Pointer {
        &self.pointer
    }
}
