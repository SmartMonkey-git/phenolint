use crate::parsing::traits::ParsableNode;
use crate::tree::node::DynamicNode;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::{
    Diagnosis, Disease, OntologyClass, PhenotypicFeature, Resource, VitalStatus,
};
use serde_json::Value;

impl ParsableNode<OntologyClass> for OntologyClass {
    fn parse(node: &DynamicNode) -> Option<OntologyClass> {
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
    fn parse(node: &DynamicNode) -> Option<PhenotypicFeature> {
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
    fn parse(node: &DynamicNode) -> Option<Phenopacket> {
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

impl ParsableNode<Resource> for Resource {
    fn parse(node: &DynamicNode) -> Option<Resource> {
        if let Value::Object(map) = &node.value
            && map.contains_key("id")
            && map.contains_key("name")
            && map.contains_key("url")
            && let Ok(resource) = serde_json::from_value::<Resource>(node.value.clone())
        {
            Some(resource)
        } else {
            None
        }
    }
}

impl ParsableNode<VitalStatus> for VitalStatus {
    fn parse(node: &DynamicNode) -> Option<VitalStatus> {
        if let Value::Object(map) = &node.value
            && map.contains_key("status")
            && let Ok(pp) = serde_json::from_value::<VitalStatus>(node.value.clone())
        {
            Some(pp)
        } else {
            None
        }
    }
}

impl ParsableNode<Disease> for Disease {
    fn parse(node: &DynamicNode) -> Option<Disease> {
        if let Value::Object(map) = &node.value
            && node
                .pointer
                .segments()
                .into_iter()
                .any(|seg| seg.to_lowercase() == "diseases")
            && map.contains_key("term")
            && let Ok(disease) = serde_json::from_value::<Disease>(node.value.clone())
        {
            Some(disease)
        } else {
            None
        }
    }
}

impl ParsableNode<Diagnosis> for Diagnosis {
    fn parse(node: &DynamicNode) -> Option<Diagnosis> {
        if let Value::Object(map) = &node.value
            && node
                .pointer
                .segments()
                .into_iter()
                .any(|seg| seg.to_lowercase() == "interpretations")
            && map.contains_key("disease")
            && let Ok(diag) = serde_json::from_value::<Diagnosis>(node.value.clone())
        {
            Some(diag)
        } else {
            None
        }
    }
}
