use crate::error::InitError;
use crate::json::Pointer;
use crate::{Node, PhenopacketNodeTraversal};
use json_spanned_value::spanned::Value as SpannedValue;
use serde_json::Value;
use std::collections::VecDeque;

pub type BoxedNode<T> = Box<dyn Node<T>>;
pub struct JsonNode {
    pub value: Value,
    pub span: (usize, usize),
    pub pointer: Pointer,
}

impl JsonNode {
    pub fn new(value: &Value, span: (usize, usize), pointer: Pointer) -> JsonNode {
        JsonNode {
            value: value.clone(),
            span,
            pointer,
        }
    }
}

impl Node<Value> for JsonNode {
    fn value(&self) -> Value {
        todo!()
    }

    fn span(&self) -> Option<(usize, usize)> {
        todo!()
    }

    fn pointer(&self) -> Pointer {
        todo!()
    }
}

pub struct PhenopacketJsonTraverser {
    phenopacket: Value,
    spans: SpannedValue,
}

impl PhenopacketJsonTraverser {
    pub fn new(json: &[u8]) -> Result<Self, InitError> {
        Ok(Self {
            phenopacket: serde_json::from_reader(json)?,
            spans: json_spanned_value::from_slice(json)?,
        })
    }
}

impl PhenopacketNodeTraversal<Value> for PhenopacketJsonTraverser {
    fn traverse(&self) -> Box<dyn Iterator<Item = Box<(dyn Node<Value> + 'static)>>> {
        let mut queue = VecDeque::new();
        let root_node = Box::new(JsonNode::new(
            &self.phenopacket,
            self.spans.span(),
            Pointer::new(""),
        )) as Box<(dyn Node<Value> + 'static)>;
        queue.push_back(root_node);

        Box::new(std::iter::from_fn(move || {
            #[allow(clippy::never_loop)]
            while let Some(current_node) = queue.pop_front() {
                match current_node.value() {
                    Value::Array(ref list) => {
                        for (i, val) in list.iter().enumerate() {
                            let mut new_pointer = current_node.pointer().clone();
                            new_pointer.down(i);

                            let next_node = JsonNode::new(
                                val,
                                self.spans
                                    .pointer(new_pointer.position())
                                    .unwrap_or_else(|| panic!("Expected spans at {}", new_pointer))
                                    .span(),
                                new_pointer,
                            );

                            queue
                                .push_back(Box::new(next_node) as Box<(dyn Node<Value> + 'static)>);
                        }
                    }
                    Value::Object(ref obj) => {
                        for (key, val) in obj {
                            let mut new_pointer = current_node.pointer().clone();
                            new_pointer.down(key);

                            let next_node = JsonNode::new(
                                val,
                                self.spans
                                    .pointer(new_pointer.position())
                                    .unwrap_or_else(|| panic!("Expected spans at {}", new_pointer))
                                    .span(),
                                new_pointer,
                            );

                            queue
                                .push_back(Box::new(next_node) as Box<(dyn Node<Value> + 'static)>);
                        }
                    }
                    _ => {}
                };

                return Some(current_node);
            }
            None
        }))
    }
}

// test Stuff

struct YamlNode {}

impl Node<String> for YamlNode {
    fn value(&self) -> String {
        todo!()
    }

    fn span(&self) -> Option<(usize, usize)> {
        todo!()
    }

    fn pointer(&self) -> Pointer {
        todo!()
    }
}
#[derive(Debug, Default)]
pub struct PhenopacketYamlTraverser;

impl PhenopacketNodeTraversal<String> for PhenopacketYamlTraverser {
    fn traverse(&self) -> Box<dyn Iterator<Item = Box<dyn Node<String>>> + '_> {
        todo!()
    }
}
