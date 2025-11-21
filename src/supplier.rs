use crate::blackboard::BlackBoard;
use crate::parsing::traits::ParsableNode;
use crate::tree::node::{DynamicNode, MaterializedNode};
use log::error;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature, VitalStatus};
use std::any::{Any, TypeId};

pub(crate) struct NodeSupplier;

impl NodeSupplier {
    pub fn supply_rules(&mut self, dyn_node: &DynamicNode, mut board: &mut BlackBoard) {
        if let Some(oc) = OntologyClass::parse(dyn_node) {
            Self::insert_into_board(oc, dyn_node, &mut board);
        } else if let Some(pf) = PhenotypicFeature::parse(dyn_node) {
            Self::insert_into_board(pf, dyn_node, &mut board);
        } else if let Some(pp) = Phenopacket::parse(dyn_node) {
            Self::insert_into_board(pp, dyn_node, &mut board);
        } else if let Some(vt) = VitalStatus::parse(dyn_node) {
            Self::insert_into_board(vt, dyn_node, &mut board);
        } else {
            error!("Unable to parse node at '{}'.", dyn_node.pointer);
        };
    }

    fn insert_into_board<T: 'static>(
        materialized: T,
        dyn_node: &DynamicNode,
        board: &mut BlackBoard,
    ) {
        let node = MaterializedNode {
            materialized_node: materialized,
            spans: dyn_node.spans.clone(),
            pointer: dyn_node.pointer.clone(),
        };
        board.insert(node);
    }
}
