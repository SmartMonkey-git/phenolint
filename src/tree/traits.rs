use crate::tree::node;
use crate::tree::node::MaterializedNode;
use crate::tree::pointer::Pointer;
use prost::Message;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::borrow::Cow;
use std::ops::Range;

pub trait UberNode: IndexNode + Node {}

impl<T> UberNode for T where T: Node + IndexNode {}

pub trait Node {
    fn span_at(&self, ptr: &Pointer) -> Option<&Range<usize>>;
    fn pointer(&self) -> &Pointer;
}

pub trait IndexNode {
    fn value_at(&'_ self, ptr: &Pointer) -> Option<Cow<'_, Value>>;
}

pub(crate) trait NodeRepository {
    fn insert<T: 'static + Message>(&mut self, node: MaterializedNode<T>) -> Result<(), String>;

    // Gets all nodes of a type in a scope

    fn get_nodes_in_scope<T: DeserializeOwned + 'static>(
        &self,
        scope: u8,
    ) -> Result<Vec<MaterializedNode<T>>, String>;

    // All nodes of a type for cases per case
    // Example: Check if all curie id's are represented in the resources in a phenopacket
    fn get_nodes_for_scope_per_top_level_element<T: Default + Message + 'static>(
        &self,
        scope: u8,
    ) -> Result<Vec<Vec<MaterializedNode<T>>>, String>;

    // Gets all nodes of a type
    // Example: Check if all CURIE id's are formatted correctly
    fn get_all<T: DeserializeOwned + 'static>(&self) -> Vec<MaterializedNode<T>>;

    // All nodes of a type for scope and lower scopes (in path)
    // All nodes of a type per top-level-scope-branch and lower scopes (in path)
}
