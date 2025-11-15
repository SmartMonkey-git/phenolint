#![allow(unused)]

use crate::tree::pointer::Pointer;
use crate::tree::span_types::Span;
use serde_json::Value;
use std::ops::Range;

#[derive()]
pub struct Node {
    pub value: Value,
    pub span: Span,
    pub pointer: Pointer,
}

impl Node {
    pub fn new(value: &Value, span: &Span, pointer: Pointer) -> Self {
        Node {
            value: value.clone(),
            span: span.clone(),
            pointer,
        }
    }
    pub fn value(&self, ptr: Pointer) -> Value {
        self.value.pointer(ptr.position()).unwrap().clone()
    }

    pub fn span(&self, ptr: &Pointer) -> Option<&Range<usize>> {
        self.span.span(ptr)
    }

    pub fn pointer(&self) -> Pointer {
        self.pointer.clone()
    }
}
