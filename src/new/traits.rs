use crate::json::Pointer;
use crate::new::node::Node;
use crate::new::tree_factory::{JsonSpan, YamlSpan};

#[derive(Clone)]
pub enum Span {
    Json(JsonSpan),
    Yaml(YamlSpan),
}

pub trait Spanning {
    fn span(&self, ptr: &Pointer) -> Option<(usize, usize)>;
}

impl Spanning for Span {
    fn span(&self, ptr: &Pointer) -> Option<(usize, usize)> {
        match self {
            Span::Json(s) => Some((1, 2)),
            Span::Yaml(s) => Some((1, 2)),
        }
    }
}

pub trait ParsableNode<N> {
    fn parse(node: &Node) -> Option<N>;
}
