use crate::tree::pointer::Pointer;
use crate::tree::traits::Node;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Range;

pub struct DynamicNode {
    value: Value,
    spans: HashMap<Pointer, Range<usize>>,
    pointer: Pointer,
}

impl DynamicNode {
    pub fn new(value: &Value, span: &HashMap<Pointer, Range<usize>>, pointer: Pointer) -> Self {
        DynamicNode {
            value: value.clone(),
            spans: span.clone(),
            pointer,
        }
    }
}

impl Node for DynamicNode {
    fn get_value(&self) -> Cow<Value> {
        Cow::Borrowed(&self.value)
    }

    fn get_value_at(&self, ptr: &Pointer) -> Option<Cow<Value>> {
        Some(Cow::Borrowed(self.value.pointer(ptr.position())?))
    }

    fn get_span(&self, ptr: &Pointer) -> Option<&Range<usize>> {
        self.spans.get(ptr)
    }

    fn pointer(&self) -> &Pointer {
        &self.pointer
    }
}

pub struct MaterializedNode<T> {
    pub materialized_node: T,
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
            materialized_node,
            spans,
            pointer,
        }
    }

    pub(crate) fn from_dyn(materialized: T, dyn_node: &DynamicNode) -> Self {
        Self::new(
            materialized,
            dyn_node.spans.clone(),
            dyn_node.pointer().clone(),
        )
    }
}

impl<T: Serialize> Node for MaterializedNode<T> {
    fn get_value(&self) -> Cow<Value> {
        let node_opt =
            serde_json::to_value(&self.materialized_node).expect("Should be serializable");
        Cow::Owned(node_opt)
    }

    fn get_value_at(&self, ptr: &Pointer) -> Option<Cow<Value>> {
        let node_opt = serde_json::to_value(&self.materialized_node).ok()?;
        let value = node_opt.pointer(ptr.position())?.clone();
        Some(Cow::Owned(value))
    }

    fn get_span(&self, ptr: &Pointer) -> Option<&Range<usize>> {
        self.spans.get(ptr)
    }

    fn pointer(&self) -> &Pointer {
        &self.pointer
    }
}
