#![allow(unused)]
use crate::tree::pointer::Pointer;
use serde_json::Value;

#[derive(Clone)]
pub struct Node {
    pub value: Value,
    pub span: (usize, usize),
    pub pointer: Pointer,
}

impl Node {
    pub fn new(value: &Value, span: &(usize, usize), pointer: Pointer) -> Self {
        Node {
            value: value.clone(),
            span: *span,
            pointer,
        }
    }
    pub fn value(&self, ptr: Pointer) -> Value {
        self.value.pointer(ptr.position()).unwrap().clone()
    }

    pub fn span(&self, ptr: Pointer) -> Option<(usize, usize)> {
        todo!()
    }

    pub fn pointer(&self) -> Pointer {
        self.pointer.clone()
    }
}
