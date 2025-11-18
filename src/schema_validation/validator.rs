use jsonref::JsonRef;
use jsonschema::ValidationError;
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

pub struct PhenopacketSchemaValidator {
    schema: Value,
}

impl PhenopacketSchemaValidator {
    /*pub fn validate<'a>(&self, phenopacket: &'a Value) -> Result<(), ValidationError<'a>> {
            match jsonschema::validate(&self.schema, &phenopacket) {
                Ok(_) => {Ok(())}
                Err(err) => {


                }
            }
        }
    */
    pub fn process_and_export_schemas() -> Result<PathBuf, Box<dyn Error>> {
        // 1. Define the list of schemas with their filenames and embedded content
        // include_str! paths are relative to this source file location
        let schemas = vec![
            (
                "phenopacket-schema.json",
                include_str!("schema/phenopacket-schema.json"),
            ),
            (
                "phenotypic-feature.json",
                include_str!("schema/phenotypic-feature.json"),
            ),
            ("pedigree.json", include_str!("schema/pedigree.json")),
            ("meta-data.json", include_str!("schema/meta-data.json")),
            (
                "medical-action.json",
                include_str!("schema/medical-action.json"),
            ),
            ("measurement.json", include_str!("schema/measurement.json")),
            (
                "interpretation.json",
                include_str!("schema/interpretation.json"),
            ),
            ("individual.json", include_str!("schema/individual.json")),
            (
                "family-schema.json",
                include_str!("schema/family-schema.json"),
            ),
            ("disease.json", include_str!("schema/disease.json")),
            (
                "cohort-schema.json",
                include_str!("schema/cohort-schema.json"),
            ),
            ("biosample.json", include_str!("schema/biosample.json")),
            (
                "base-recommended.json",
                include_str!("schema/base-recommended.json"),
            ),
            ("base.json", include_str!("schema/base.json")),
        ];
        let temp_dir = std::env::temp_dir().join("phenopackets_schemas_processed");

        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir)?;
        }
        fs::create_dir_all(&temp_dir)?;

        println!("Processing schemas to: {:?}", temp_dir);

        // 3. Iterate, Replace, and Write
        for (filename, content) in schemas {
            // Apply the path replacement
            let fixed_content = content.replace(
                "classpath:/org/phenopackets/phenopackettools/validator/jsonschema/v2/",
                "",
            );

            let file_path = temp_dir.join(filename);
            let mut file = fs::File::create(file_path)?;
            file.write_all(fixed_content.as_bytes())?;
        }

        Ok(temp_dir)
    }
}

impl Default for PhenopacketSchemaValidator {
    fn default() -> Self {
        let temp_dir = Self::process_and_export_schemas().unwrap();
        let schema_dir = temp_dir.join("phenopacket-schema.json");
        let s = fs::read_to_string(schema_dir).unwrap();
        let mut schema_json: Value = serde_json::from_str(&s).expect("Schema is not valid JSON");

        let mut jsonref = JsonRef::new();

        jsonref.deref_value(&mut schema_json).unwrap();

        Self {
            schema: schema_json,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::patches::enums::PatchInstruction;
    use crate::patches::patch::Patch;
    use crate::patches::patch_engine::PatchEngine;
    use crate::tree::pointer::Pointer;
    use rstest::rstest;
    use serde_json::{Number, Value, json};

    #[rstest]
    fn test_init() {
        let validator = PhenopacketSchemaValidator::default();
    }
}
