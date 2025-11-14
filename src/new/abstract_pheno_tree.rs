use crate::json::Pointer;
use crate::new::node::Node;
use crate::new::span_types::Span;
use crate::new::traits::Spanning;
use serde_json::Value;
use std::collections::VecDeque;

pub struct AbstractPhenoTree {
    tree: Value,
    spans: Span,
}

impl AbstractPhenoTree {
    pub(crate) fn new(tree: Value, spans: Span) -> AbstractPhenoTree {
        AbstractPhenoTree { tree, spans }
    }

    pub(crate) fn traverse<'s>(&'s self) -> Box<dyn Iterator<Item = Node> + 's> {
        let mut queue = VecDeque::new();
        let root_node = Node::new(
            &self.tree,
            &self.spans.span(&Pointer::at_root()).unwrap(),
            Pointer::new(""),
        );
        queue.push_back(root_node);

        Box::new(std::iter::from_fn(move || {
            #[allow(clippy::never_loop)]
            while let Some(current_node) = queue.pop_front() {
                match current_node.value(Pointer::at_root()) {
                    Value::Array(ref list) => {
                        for (i, val) in list.iter().enumerate() {
                            let mut new_pointer = current_node.pointer().clone();
                            new_pointer.down(i);

                            let next_node = Node::new(
                                val,
                                &self
                                    .spans
                                    .span(&new_pointer)
                                    .unwrap_or_else(|| panic!("Expected spans at {}", new_pointer)),
                                new_pointer,
                            );

                            queue.push_back(next_node);
                        }
                    }
                    Value::Object(ref obj) => {
                        for (key, val) in obj {
                            let mut new_pointer = current_node.pointer().clone();
                            new_pointer.down(key);

                            let next_node = Node::new(
                                val,
                                &self
                                    .spans
                                    .span(&new_pointer)
                                    .unwrap_or_else(|| panic!("Expected spans at {}", new_pointer)),
                                new_pointer,
                            );

                            queue.push_back(next_node);
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
