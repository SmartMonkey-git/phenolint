use crate::NodeParser;
use crate::new::json_traverser::{BoxedNode, JsonNode};
use phenopackets::schema::v2::core::OntologyClass;
use serde_json::Value;

struct JsonBuildingBlockParser;

impl NodeParser<Value> for JsonBuildingBlockParser {
    fn parse_ontology_class(value: &BoxedNode<Value>) -> Option<OntologyClass> {
        if let Value::Object(map) = &value.value()
            && map.keys().len() == 2
            && map.contains_key("label")
            && map.contains_key("id")
            && let Ok(ont_class) = serde_json::from_value::<OntologyClass>(value.value())
        {
            Some(ont_class)
        } else {
            None
        }
    }
}
