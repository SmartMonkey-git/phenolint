use crate::tree::node::DynamicNode;

pub trait ParsableNode<N> {
    fn parse(node: &DynamicNode) -> Option<N>;
}
