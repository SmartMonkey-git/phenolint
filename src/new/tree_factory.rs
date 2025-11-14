use crate::error::InitError;
use crate::new::abstract_pheno_tree::AbstractPhenoTree;
use crate::new::traits::Span;
use phenopackets::schema::v2::Phenopacket;
use prost::Message;
use spanned_json_parser::SpannedValue;
use spanned_json_parser::parse;

#[derive(Debug, Clone)]
pub struct JsonSpan {
    spans: SpannedValue,
}

impl JsonSpan {
    fn new(spans: SpannedValue) -> JsonSpan {
        JsonSpan { spans }
    }
}
#[derive(Debug, Clone)]
pub struct YamlSpan {
    spans: Vec<(usize, usize)>,
}

impl YamlSpan {
    fn new(spans: Vec<(usize, usize)>) -> YamlSpan {
        YamlSpan { spans }
    }
}

/// Tries to parse any tree like structure that is implemented into a serde_json::Value
#[derive(Debug, Default)]
pub struct TreeFactory;

impl TreeFactory {
    pub fn try_parse(phenobytes: &[u8]) -> Result<AbstractPhenoTree, InitError> {
        //TODO: Better error reporting
        if let Ok(json) = Self::try_to_json(phenobytes) {
            println!("Going with json");
            return Ok(json);
        } else if let Ok(yaml) = Self::try_to_yaml(phenobytes) {
            println!("Going with yaml");
            return Ok(yaml);
        } else if let Ok(pb) = Self::try_to_protobuf(phenobytes) {
            println!("Going with protobuf");
            return Ok(pb);
        }

        Err(InitError::Unparseable)
    }

    fn try_to_json(phenobytes: &[u8]) -> Result<AbstractPhenoTree, InitError> {
        let json_string = String::from_utf8(phenobytes.to_vec())?;

        if let Ok(json) = serde_json::from_str(&json_string)
            && let Ok(spans) = parse(&json_string)
        {
            return Ok(AbstractPhenoTree::new(
                json,
                Span::Json(JsonSpan::new(spans)),
            ));
        }
        Err(InitError::Unparseable)
    }

    fn try_to_yaml(phenobytes: &[u8]) -> Result<AbstractPhenoTree, InitError> {
        if let Ok(yaml) = serde_yaml::from_slice(phenobytes) {
            return Ok(AbstractPhenoTree::new(
                yaml,
                Span::Yaml(YamlSpan::new(vec![(3, 4)])),
            ));
        }
        Err(InitError::Unparseable)
    }

    fn try_to_protobuf(phenobytes: &[u8]) -> Result<AbstractPhenoTree, InitError> {
        let pp = Phenopacket::decode(phenobytes)?;
        let json_string = serde_json::to_string_pretty(&pp)?;

        if let Ok(json) = serde_json::from_str(&json_string)
            && let Ok(spans) = parse(&json_string)
        {
            return Ok(AbstractPhenoTree::new(
                json,
                Span::Json(JsonSpan::new(spans)),
            ));
        }
        Err(InitError::Unparseable)
    }
}
