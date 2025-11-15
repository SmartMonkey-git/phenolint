use crate::error::InitError;
use crate::parsing::utils::{collect_json_spans, collect_yaml_spans};
use crate::tree::abstract_pheno_tree::AbstractPhenoTree;
use phenopackets::schema::v2::Phenopacket;
use prost::Message;

pub struct PhenopacketParser;

// TODO: Find logical naming for the function. Try to avoid duplicate code.
impl PhenopacketParser {
    pub fn to_tree(phenobytes: &[u8]) -> Result<AbstractPhenoTree, InitError> {
        //TODO: Better error reporting
        if let Ok(json) = Self::try_to_json_tree(phenobytes) {
            return Ok(json);
        } else if let Ok(yaml) = Self::try_to_yaml_tree(phenobytes) {
            return Ok(yaml);
        } else if let Ok(pb) = Self::try_to_protobuf_tree(phenobytes) {
            return Ok(pb);
        }

        Err(InitError::Unparseable)
    }

    fn try_to_json_tree(phenobytes: &[u8]) -> Result<AbstractPhenoTree, InitError> {
        let json_string = String::from_utf8(phenobytes.to_vec())?;

        if let Ok(json) = serde_json::from_str(&json_string)
            && let Ok(spans) = collect_json_spans(&json_string)
        {
            return Ok(AbstractPhenoTree::new(json, spans));
        }
        Err(InitError::Unparseable)
    }

    fn try_to_yaml_tree(phenobytes: &[u8]) -> Result<AbstractPhenoTree, InitError> {
        let yaml_string = String::from_utf8(phenobytes.to_vec())?;
        if let Ok(yaml) = serde_yaml::from_str(&yaml_string)
            && let Ok(spans) = collect_yaml_spans(&yaml_string)
        {
            return Ok(AbstractPhenoTree::new(yaml, spans));
        }
        Err(InitError::Unparseable)
    }

    fn try_to_protobuf_tree(phenobytes: &[u8]) -> Result<AbstractPhenoTree, InitError> {
        let json_string = Self::try_from_protobuf(phenobytes)?;

        if let Ok(json) = serde_json::from_str(&json_string)
            && let Ok(spans) = collect_json_spans(&json_string)
        {
            return Ok(AbstractPhenoTree::new(json, spans));
        }
        Err(InitError::Unparseable)
    }

    pub fn to_string(phenobytes: &[u8]) -> Result<String, InitError> {
        if let Ok(json_str) = Self::try_from_json(phenobytes) {
            Ok(json_str)
        } else if let Ok(yaml) = Self::try_from_yaml(phenobytes) {
            Ok(yaml)
        } else if let Ok(pb) = Self::try_from_protobuf(phenobytes) {
            Ok(pb)
        } else {
            Err(InitError::Unparseable)
        }
    }

    fn try_from_json(phenobytes: &[u8]) -> Result<String, InitError> {
        Ok(serde_json::from_slice::<String>(phenobytes)?)
    }

    fn try_from_yaml(phenobytes: &[u8]) -> Result<String, InitError> {
        Ok(String::from_utf8(phenobytes.to_vec())?)
    }

    fn try_from_protobuf(phenobytes: &[u8]) -> Result<String, InitError> {
        let pp = Phenopacket::decode(phenobytes)?;
        Ok(serde_json::to_string_pretty(&pp)?)
    }
}
