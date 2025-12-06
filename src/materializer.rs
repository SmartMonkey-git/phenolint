use crate::parsing::traits::ParsableNode;
use crate::tree::node::{DynamicNode, MaterializedNode};
use crate::tree::sql_node_repository::SQLNodeRepository;
use crate::tree::traits::{Node, NodeRepository};
use log::error;
use phenopackets::schema::v2::core::{
    Diagnosis, Disease, OntologyClass, PhenotypicFeature, Resource, VitalStatus,
};
use phenopackets::schema::v2::{Cohort, Phenopacket};
use prost::Message;
use serde::Serialize;

pub(crate) struct NodeMaterializer;

impl NodeMaterializer {
    pub fn materialize_nodes(&self, dyn_node: &DynamicNode, repo: &mut SQLNodeRepository) {
        if let Some(cohort) = Cohort::parse(dyn_node) {
            Self::push_to_repo(cohort, dyn_node, repo);
        } else if let Some(oc) = OntologyClass::parse(dyn_node) {
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

    fn push_to_repo<T: 'static + Message>(
        materialized: T,
        dyn_node: &DynamicNode,
        board: &mut SQLNodeRepository,
    ) {
        let node = MaterializedNode::from_dynamic(materialized, dyn_node);
        board.insert(node).expect("TODO: panic message");
    }
}
