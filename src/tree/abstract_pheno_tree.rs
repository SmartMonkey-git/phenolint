use crate::tree::node::DynamicNode;
use crate::tree::pointer::Pointer;
use crate::tree::traits::Node;
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::ops::Range;

pub struct AbstractTreeTraversal {
    tree: Value,
    spans: HashMap<Pointer, Range<usize>>,
}

impl AbstractTreeTraversal {
    pub fn new(tree: &Value, spans: &HashMap<Pointer, Range<usize>>) -> AbstractTreeTraversal {
        AbstractTreeTraversal {
            tree: tree.clone(),
            spans: spans.clone(),
        }
    }

    pub fn traverse<'s>(&'s self) -> Box<dyn Iterator<Item = DynamicNode> + 's> {
        let mut queue = VecDeque::new();
        let root_node = DynamicNode::new(&self.tree, &self.spans.clone(), Pointer::at_root());
        queue.push_back(root_node);

        Box::new(std::iter::from_fn(move || {
            #[allow(clippy::never_loop)]
            while let Some(current_node) = queue.pop_front() {
                if let Some(value) = current_node.get_value_at(&Pointer::at_root()) {
                    match value.as_ref() {
                        Value::Array(list) => {
                            for (i, val) in list.iter().enumerate() {
                                let mut new_pointer = current_node.pointer().clone();
                                new_pointer.down(i);

                                let next_node =
                                    DynamicNode::new(val, &self.spans.clone(), new_pointer);

                                queue.push_back(next_node);
                            }
                        }
                        Value::Object(obj) => {
                            for (key, val) in obj {
                                let mut new_pointer = current_node.pointer().clone();
                                new_pointer.down(key);

                                let next_node =
                                    DynamicNode::new(val, &self.spans.clone(), new_pointer);

                                queue.push_back(next_node);
                            }
                        }
                        _ => {}
                    }
                }

                return Some(current_node);
            }
            None
        }))
    }
}
