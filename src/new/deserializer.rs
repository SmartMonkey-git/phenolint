use crate::DeserializePhenopackets;
use phenopackets::schema::v2::Phenopacket;
use serde::Deserialize;
use serde_json::Value;

pub struct PhenopacketsDeserializer;

impl DeserializePhenopackets<Value> for PhenopacketsDeserializer {
    fn deserialize(bytes: &[u8]) -> Value {
        todo!()
    }
}

// Dummy YAML implementation
impl DeserializePhenopackets<String> for PhenopacketsDeserializer {
    fn deserialize(bytes: &[u8]) -> String {
        todo!()
    }
}
