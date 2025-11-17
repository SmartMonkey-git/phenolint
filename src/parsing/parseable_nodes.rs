use crate::parsing::traits::ParsableNode;
use crate::tree::node::Node;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature};
use serde_json::Value;

impl ParsableNode<OntologyClass> for OntologyClass {
    fn parse(node: &Node) -> Option<OntologyClass> {
        if let Value::Object(map) = &node.value
            && map.keys().len() == 2
            && map.contains_key("label")
            && map.contains_key("id")
            && let Ok(ont_class) = serde_json::from_value::<OntologyClass>(node.value.clone())
        {
            Some(ont_class)
        } else {
            None
        }
    }
}

impl ParsableNode<PhenotypicFeature> for PhenotypicFeature {
    fn parse(node: &Node) -> Option<PhenotypicFeature> {
        if let Value::Object(map) = &node.value
            && map.contains_key("type")
            && let Ok(phenotypic_feature) =
                serde_json::from_value::<PhenotypicFeature>(node.value.clone())
        {
            Some(phenotypic_feature)
        } else {
            None
        }
    }
}

impl ParsableNode<Phenopacket> for Phenopacket {
    fn parse(node: &Node) -> Option<Phenopacket> {
        if let Value::Object(map) = &node.value
            && map.contains_key("id")
            && map.contains_key("metaData")
            && node.pointer.is_root()
            && let Ok(pp) = serde_json::from_value::<Phenopacket>(node.value.clone())
        {
            Some(pp)
        } else {
            None
        }
    }
}
