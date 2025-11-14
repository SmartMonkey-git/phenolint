use crate::tree::node::Node;

pub trait ParsableNode<N> {
    fn parse(node: &Node) -> Option<N>;
}
