use crate::parsing::traits::ParsableNode;
use crate::tree::node::{DynamicNode, MaterializedNode};
use crate::tree::node_repository::NodeRepository;
use log::error;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature, VitalStatus};

pub(crate) struct NodeMaterializer;

impl NodeMaterializer {
    pub fn materialize_nodes(&mut self, dyn_node: &DynamicNode, repo: &mut NodeRepository) {
        if let Some(oc) = OntologyClass::parse(dyn_node) {
            Self::push_to_repo(oc, dyn_node, repo);
        } else if let Some(pf) = PhenotypicFeature::parse(dyn_node) {
            Self::push_to_repo(pf, dyn_node, repo);
        } else if let Some(pp) = Phenopacket::parse(dyn_node) {
            Self::push_to_repo(pp, dyn_node, repo);
        } else if let Some(vt) = VitalStatus::parse(dyn_node) {
            Self::push_to_repo(vt, dyn_node, repo);
        } else {
            error!("Unable to parse node at '{}'.", dyn_node.pointer);
        };
    }

    fn push_to_repo<T: 'static>(
        materialized: T,
        dyn_node: &DynamicNode,
        board: &mut NodeRepository,
    ) {
        let node = MaterializedNode {
            materialized_node: materialized,
            spans: dyn_node.spans.clone(),
            pointer: dyn_node.pointer.clone(),
        };
        board.insert(node);
    }
}
