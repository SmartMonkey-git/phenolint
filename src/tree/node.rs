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

#[derive(Clone, Debug)]
pub struct MaterializedNode<NodeType> {
    pub inner: NodeType,
    spans: HashMap<Pointer, Range<usize>>,
    pointer: Pointer,
}

impl<NodeType> MaterializedNode<NodeType> {
    pub fn new(
        materialized_node: NodeType,
        spans: HashMap<Pointer, Range<usize>>,
        pointer: Pointer,
    ) -> Self {
        MaterializedNode {
            inner: materialized_node,
            spans,
            pointer,
        }
    }

    pub(crate) fn from_dynamic(materialized: NodeType, dyn_node: &DynamicNode) -> Self {
        Self::new(
            materialized,
            dyn_node.spans.clone(),
            dyn_node.pointer().clone(),
        )
    }

    #[allow(dead_code)]
    pub(crate) fn spans(&self) -> &HashMap<Pointer, Range<usize>> {
        &self.spans
    }
}

impl<NodeType: Serialize> RetrievableNode for MaterializedNode<NodeType> {
    fn value_at(&self, ptr: &Pointer) -> Option<Cow<'_, Value>> {
        let node_opt = serde_json::to_value(&self.inner).ok()?;
        let value = node_opt.pointer(ptr.position())?.clone();
        Some(Cow::Owned(value))
    }
}

impl<NodeType> LocatableNode for MaterializedNode<NodeType> {
    fn span_at(&self, ptr: &Pointer) -> Option<&Range<usize>> {
        self.spans.get(ptr)
    }

    fn pointer(&self) -> &Pointer {
        &self.pointer
    }
}
