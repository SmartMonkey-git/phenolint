use crate::parsing::traits::ParsableNode;
use crate::tree::node::{DynamicNode, MaterializedNode};
use crate::tree::node_repository::NodeRepository;
use crate::tree::traits::Node;
use log::error;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::{
    Diagnosis, Disease, OntologyClass, PhenotypicFeature, Resource, VitalStatus,
};
use serde::Serialize;

pub(crate) struct NodeMaterializer;

// Scoped -> Would give you the desired classes for each top level element
// Global -> Would give you all objects for a desired class
// List and Single -> Give you either one or all

// Use a trait called Scopable and implement for Phenopacket, Cohort and Family

// Scope should only be allowed for top level elements
// DataAccess<Scope<Phenopacket>, List<'a, OntologyClass>>
// DataAccess<Scope<Cohort>, List<'a, OntologyClass>>
// DataAccess<Scope<Phenopacket>, Single<'a, Option<Cohort>> ❌
impl NodeMaterializer {
    pub fn materialize_nodes(&mut self, dyn_node: &DynamicNode, repo: &mut NodeRepository) {
        // TODO: This needs to be reduced if-branches for the TOP-Level elements
        // At the end it needs to produce a HashMap<TypeId, Box<dyn Any>> for each top level element
        if let Some(oc) = OntologyClass::parse(dyn_node) {
            Self::push_to_repo(oc, dyn_node, repo);
        } else if let Some(pf) = PhenotypicFeature::parse(dyn_node) {
            Self::push_to_repo(pf, dyn_node, repo);
        } else if let Some(pp) = Phenopacket::parse(dyn_node) {
            Self::push_to_repo(pp, dyn_node, repo);
        } else if let Some(vt) = VitalStatus::parse(dyn_node) {
            Self::push_to_repo(vt, dyn_node, repo);
        } else if let Some(resource) = Resource::parse(dyn_node) {
            Self::push_to_repo(resource, dyn_node, repo);
        } else if let Some(resource) = Disease::parse(dyn_node) {
            Self::push_to_repo(resource, dyn_node, repo);
        } else if let Some(resource) = Diagnosis::parse(dyn_node) {
            Self::push_to_repo(resource, dyn_node, repo);
        } else {
            error!("Unable to parse node at '{}'.", dyn_node.pointer());
        };
    }

    fn push_to_repo<T: 'static + Clone + Serialize>(
        materialized: T,
        dyn_node: &DynamicNode,
        board: &mut NodeRepository,
    ) {
        let node = MaterializedNode::from_dynamic(materialized, dyn_node);
        board.insert(node);
    }
}
