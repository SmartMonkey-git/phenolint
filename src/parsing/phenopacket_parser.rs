use crate::error::ParsingError;
use crate::parsing::utils::{collect_json_spans, collect_yaml_spans};
use crate::tree::abstract_pheno_tree::AbstractPhenoTree;
use phenopackets::schema::v2::Phenopacket;
use prost::Message;

pub struct PhenopacketParser;

// TODO: Find logical naming for the function. Try to avoid duplicate code.
impl PhenopacketParser {
    pub fn to_tree(phenostr: &str) -> Result<AbstractPhenoTree, ParsingError> {
        //TODO: Better error reporting
        if let Ok(json) = Self::try_to_json_tree(phenostr) {
            return Ok(json);
        } else if let Ok(yaml) = Self::try_to_yaml_tree(phenostr) {
            return Ok(yaml);
        } else if let Ok(pb) = Self::try_to_protobuf_tree(phenostr) {
            return Ok(pb);
        }

        Err(ParsingError::Unparseable)
    }

    fn try_to_json_tree(phenostr: &str) -> Result<AbstractPhenoTree, ParsingError> {
        if let Ok(json) = serde_json::from_str(&phenostr)
            && let Ok(spans) = collect_json_spans(&phenostr)
        {
            return Ok(AbstractPhenoTree::new(json, spans));
        }
        Err(ParsingError::Unparseable)
    }

    fn try_to_yaml_tree(phenostr: &str) -> Result<AbstractPhenoTree, ParsingError> {
        if let Ok(yaml) = serde_yaml::from_str(&phenostr)
            && let Ok(spans) = collect_yaml_spans(&phenostr)
        {
            return Ok(AbstractPhenoTree::new(yaml, spans));
        }
        Err(ParsingError::Unparseable)
    }

    fn try_to_protobuf_tree(phenostr: &str) -> Result<AbstractPhenoTree, ParsingError> {
        if let Ok(json) = serde_json::from_str(&phenostr)
            && let Ok(spans) = collect_json_spans(&phenostr)
        {
            return Ok(AbstractPhenoTree::new(json, spans));
        }
        Err(ParsingError::Unparseable)
    }

    pub fn to_string(phenobytes: &[u8]) -> Result<String, ParsingError> {
        if let Ok(json_str) = Self::try_from_json(phenobytes) {
            Ok(json_str)
        } else if let Ok(yaml) = Self::try_from_yaml(phenobytes) {
            Ok(yaml)
        } else if let Ok(pb) = Self::try_from_protobuf(phenobytes) {
            Ok(pb)
        } else {
            Err(ParsingError::Unparseable)
        }
    }

    fn try_from_json(phenobytes: &[u8]) -> Result<String, ParsingError> {
        Ok(serde_json::from_slice::<String>(phenobytes)?)
    }

    fn try_from_yaml(phenobytes: &[u8]) -> Result<String, ParsingError> {
        Ok(String::from_utf8(phenobytes.to_vec())?)
    }

    fn try_from_protobuf(phenobytes: &[u8]) -> Result<String, ParsingError> {
        let pp = Phenopacket::decode(phenobytes)?;
        Ok(serde_json::to_string_pretty(&pp)?)
    }
}
