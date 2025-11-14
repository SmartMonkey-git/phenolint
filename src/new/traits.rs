use crate::json::Pointer;
use crate::new::node::Node;

pub trait Spanning {
    fn span(&self, ptr: &Pointer) -> Option<(usize, usize)>;
}

pub trait ParsableNode<N> {
    fn parse(node: &Node) -> Option<N>;
}
