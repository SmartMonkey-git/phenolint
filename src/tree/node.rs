#![allow(unused)]

use crate::tree::pointer::Pointer;
use serde_json::Value;
use std::any::Any;
use std::collections::HashMap;
use std::ops::Range;

#[derive()]
pub struct DynamicNode {
    pub value: Value,
    pub spans: HashMap<Pointer, Range<usize>>,
    pub pointer: Pointer,
}

impl DynamicNode {
    pub fn new(value: &Value, span: &HashMap<Pointer, Range<usize>>, pointer: Pointer) -> Self {
        DynamicNode {
            value: value.clone(),
            spans: span.clone(),
            pointer,
        }
    }
    pub fn value(&self, ptr: &Pointer) -> Value {
        self.value.pointer(ptr.position()).unwrap().clone()
    }

    pub fn span(&self, ptr: &Pointer) -> Option<&Range<usize>> {
        self.spans.get(ptr)
    }

    pub fn pointer(&self) -> Pointer {
        self.pointer.clone()
    }
}

pub struct MaterializedNode {
    pub materialized_node: Box<dyn Any>,
    pub spans: HashMap<Pointer, Range<usize>>,
    pub pointer: Pointer,
}
