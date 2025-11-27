use crate::tree::pointer::Pointer;
use crate::tree::traits::Node;
use serde::Serialize;
use serde_json::Value;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Range;
use std::process::Output;

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
    type Output = Value;
    fn get_node(&self) -> &Self::Output {
        &self.value
    }

    fn get_value_at(&'_ self, ptr: &Pointer) -> Option<Cow<'_, Value>> {
        Some(Cow::Borrowed(self.value.pointer(ptr.position())?))
    }

    fn get_span(&self, ptr: &Pointer) -> Option<&Range<usize>> {
        self.spans.get(ptr)
    }

    fn pointer(&self) -> &Pointer {
        &self.pointer
    }
}

pub struct MaterializedNode<T>
where
    T: Clone + Serialize,
{
    pub materialized_node: T,
    spans: HashMap<Pointer, Range<usize>>,
    pointer: Pointer,
}

impl<T: Clone + Serialize> MaterializedNode<T> {
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

impl<T: Clone + Serialize> Node for MaterializedNode<T> {
    type Output = T;
    fn get_node(&self) -> &Self::Output {
        &self.materialized_node
    }

    fn get_value_at(&'_ self, ptr: &Pointer) -> Option<Cow<'_, Value>> {
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
