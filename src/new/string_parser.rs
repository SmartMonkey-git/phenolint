use crate::error::InitError;
use phenopackets::schema::v2::Phenopacket;
use prost::Message;

pub struct StringParser;

impl StringParser {
    pub fn parse_phenopacket_to_string(phenobytes: &[u8]) -> Result<String, InitError> {
        if let Ok(json_str) = Self::try_from_json(phenobytes) {
            Ok(json_str)
        } else {
            Err(InitError::Unparseable)
        }
    }

    fn try_from_json(phenobytes: &[u8]) -> Result<String, InitError> {
        Ok(serde_json::from_slice::<String>(phenobytes)?)
    }

    fn try_from_yaml(phenobytes: &[u8]) -> Result<String, InitError> {
        Ok(serde_yaml::from_slice::<String>(phenobytes)?)
    }

    fn try_from_protobuf(phenobytes: &[u8]) -> Result<String, InitError> {
        let pp = Phenopacket::decode(phenobytes)?;
        Ok(serde_json::to_string_pretty(&pp)?)
    }
}
