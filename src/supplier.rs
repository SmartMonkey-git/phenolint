use crate::parsing::traits::ParsableNode;
use crate::tree::node::{DynamicNode, MaterializedNode};
use log::error;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature, VitalStatus};
use std::any::{Any, TypeId};

pub(crate) struct NodeSupplier;

impl NodeSupplier {
    pub fn supply_rules(&mut self, dyn_node: &DynamicNode) -> Option<MaterializedNode> {
        let materialized: Box<dyn Any> = if let Some(oc) = OntologyClass::parse(dyn_node) {
            Box::new(oc)
        } else if let Some(pf) = PhenotypicFeature::parse(dyn_node) {
            Box::new(pf)
        } else if let Some(pp) = Phenopacket::parse(dyn_node) {
            Box::new(pp)
        } else if let Some(vt) = VitalStatus::parse(dyn_node) {
            Box::new(vt)
        } else {
            error!("Unable to parse node at '{}'.", dyn_node.pointer);
            return None;
        };

        Some(MaterializedNode {
            materialized_node: materialized,
            spans: dyn_node.spans.clone(),
            pointer: dyn_node.pointer.clone(),
        })
    }
}
