use crate::parsing::traits::ParsableNode;
use crate::tree::node::DynamicNode;
use crate::tree::traits::Node;
use phenopackets::schema::v2::core::{
    Diagnosis, Disease, OntologyClass, PhenotypicFeature, Resource, VitalStatus,
};
use phenopackets::schema::v2::{Cohort, Phenopacket};
use serde_json::Value;

impl ParsableNode<OntologyClass> for OntologyClass {
    fn parse(node: &DynamicNode) -> Option<OntologyClass> {
        if let Value::Object(map) = &node.inner
            && map.keys().len() == 2
            && map.contains_key("label")
            && map.contains_key("id")
            && let Ok(ont_class) = serde_json::from_value::<OntologyClass>(node.inner.clone())
        {
            Some(ont_class)
        } else {
            None
        }
    }
}

impl ParsableNode<PhenotypicFeature> for PhenotypicFeature {
    fn parse(node: &DynamicNode) -> Option<PhenotypicFeature> {
        if let Value::Object(map) = &node.inner
            && map.contains_key("type")
            && let Ok(phenotypic_feature) =
                serde_json::from_value::<PhenotypicFeature>(node.inner.clone())
        {
            Some(phenotypic_feature)
        } else {
            None
        }
    }
}

impl ParsableNode<Phenopacket> for Phenopacket {
    fn parse(node: &DynamicNode) -> Option<Phenopacket> {
        if let Value::Object(map) = &node.inner
            && map.contains_key("id")
            && map.contains_key("metaData")
            && let Ok(pp) = serde_json::from_value::<Phenopacket>(node.inner.clone())
        {
            Some(pp)
        } else {
            None
        }
    }
}

impl ParsableNode<Cohort> for Cohort {
    fn parse(node: &DynamicNode) -> Option<Cohort> {
        if let Value::Object(map) = &node.inner
            && map.contains_key("id")
            && map.contains_key("members")
            && let Ok(cohort) = serde_json::from_value::<Cohort>(node.inner.clone())
        {
            Some(cohort)
        } else {
            None
        }
    }
}

impl ParsableNode<Resource> for Resource {
    fn parse(node: &DynamicNode) -> Option<Resource> {
        if let Value::Object(map) = &node.inner
            && map.contains_key("id")
            && map.contains_key("name")
            && map.contains_key("url")
            && let Ok(resource) = serde_json::from_value::<Resource>(node.inner.clone())
        {
            Some(resource)
        } else {
            None
        }
    }
}

impl ParsableNode<VitalStatus> for VitalStatus {
    fn parse(node: &DynamicNode) -> Option<VitalStatus> {
        if let Value::Object(map) = &node.inner
            && map.contains_key("status")
            && let Ok(pp) = serde_json::from_value::<VitalStatus>(node.inner.clone())
        {
            Some(pp)
        } else {
            None
        }
    }
}

impl ParsableNode<Disease> for Disease {
    fn parse(node: &DynamicNode) -> Option<Disease> {
        if let Value::Object(map) = &node.inner
            && node
                .pointer()
                .segments()
                .into_iter()
                .any(|seg| seg.to_lowercase() == "diseases")
            && map.contains_key("term")
            && let Ok(disease) = serde_json::from_value::<Disease>(node.inner.clone())
        {
            Some(disease)
        } else {
            None
        }
    }
}

impl ParsableNode<Diagnosis> for Diagnosis {
    fn parse(node: &DynamicNode) -> Option<Diagnosis> {
        if let Value::Object(map) = &node.inner
            && node
                .pointer()
                .segments()
                .into_iter()
                .any(|seg| seg.to_lowercase() == "interpretations")
            && map.contains_key("disease")
            && let Ok(diag) = serde_json::from_value::<Diagnosis>(node.inner.clone())
        {
            Some(diag)
        } else {
            None
        }
    }
}
