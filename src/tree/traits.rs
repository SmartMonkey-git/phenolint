#![allow(dead_code)]
use crate::tree::error::NodeRepositoryError;
use crate::tree::node::MaterializedNode;
use crate::tree::pointer::Pointer;
use crate::tree::scopes::ScopeLayer;
use serde_json::Value;
use std::borrow::Cow;
use std::ops::Range;

pub trait Node: LocatableNode + RetrievableNode {}

impl<T: LocatableNode + RetrievableNode> Node for T {}

pub trait LocatableNode {
    fn span_at(&self, ptr: &Pointer) -> Option<&Range<usize>>;
    fn pointer(&self) -> &Pointer;
}

pub trait RetrievableNode {
    fn value_at(&self, ptr: &Pointer) -> Option<Cow<'_, Value>>;
}

pub trait NodeRepository {
    fn insert<T: 'static + Clone>(
        &mut self,
        node: MaterializedNode<T>,
    ) -> Result<(), NodeRepositoryError>;

    // Gets all nodes of a type
    // Example: Check if all CURIE id's are formatted correctly
    fn get_all<T: Clone + 'static>(&self) -> Result<Vec<MaterializedNode<T>>, NodeRepositoryError>;

    // Gets all nodes of a type in a scope
    // Example: Get all nodes of the phenopacket scope + all resources of the cohort. Check if pp resources are in cohort
    fn get_nodes_in_scope<T: Clone + 'static>(
        &self,
        scope: ScopeLayer,
    ) -> Result<Vec<MaterializedNode<T>>, NodeRepositoryError>;

    // All nodes of a type for cases per case
    // Example: Check if all curie id's are represented in the resources in a phenopacket
    fn get_nodes_for_scope_per_top_level_element<T: Clone + 'static>(
        &self,
        scope: ScopeLayer,
    ) -> Result<Vec<Vec<MaterializedNode<T>>>, NodeRepositoryError>;
}
