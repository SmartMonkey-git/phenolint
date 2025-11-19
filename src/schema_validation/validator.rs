#![allow(dead_code)]
use jsonschema::{Registry, Resource, ValidationError, Validator};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub struct PhenopacketSchemaValidator {
    schema: Validator,
}

impl PhenopacketSchemaValidator {
    pub fn validate_phenopacket<'i>(
        &self,
        phenopacket: &'i Value,
    ) -> Result<(), Box<ValidationError<'i>>> {
        self.schema.validate(phenopacket).map_err(Box::new)
    }

    fn process_and_export_schemas() -> Result<HashMap<String, Resource>, Box<dyn Error>> {
        let schemas = Self::schema_definitions();

        schemas
            .into_iter()
            .map(|(filename, content)| {
                let resource = Self::create_resource(content)?;
                Ok((filename, resource))
            })
            .collect()
    }

    fn schema_definitions() -> Vec<(String, String)> {
        vec![
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
            (
                "vrs-variation-adapter.json",
                include_str!("schema/vrs-variation-adapter.json"),
            ),
            ("vrsatile.json", include_str!("schema/vrsatile.json")),
        ]
        .into_iter()
        .map(|(name, content)| (name.to_string(), content.to_string()))
        .collect()
    }

    /// Create a resource from schema content
    fn create_resource(content: String) -> Result<Resource, Box<dyn Error>> {
        let cleaned = Self::normalize_schema_refs(&content);
        let mut value: Value = serde_json::from_str(&cleaned)?;

        Self::remove_id_field(&mut value);
        Ok(Resource::from_contents(value))
    }

    fn normalize_schema_refs(content: &str) -> String {
        content
            .replace(
                "classpath:/org/phenopackets/phenopackettools/validator/jsonschema/v2/",
                "",
            )
            .replace(
                "classpath:/org/phenopackets/phenopackettools/validator/jsonschema/",
                "",
            )
    }

    fn remove_id_field(value: &mut Value) {
        if let Some(obj) = value.as_object_mut() {
            obj.remove("$id");
        }
    }

    fn build_main_schema(registry: Registry) -> Result<Validator, Box<dyn Error>> {
        let main_schema = include_str!("schema/phenopacket-schema.json");
        let cleaned = Self::normalize_schema_refs(main_schema);
        let mut value: Value = serde_json::from_str(&cleaned)?;

        Self::remove_id_field(&mut value);

        jsonschema::options()
            .with_registry(registry)
            .build(&value)
            .map_err(Into::into)
    }
}

impl Default for PhenopacketSchemaValidator {
    fn default() -> Self {
        let resources = Self::process_and_export_schemas().expect("Failed to process schemas");

        let registry =
            Registry::try_from_resources(resources).expect("Failed to create schema registry");

        let schema = Self::build_main_schema(registry).expect("Failed to build main schema");

        Self { schema }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::json_phenopacket_dir;
    use rstest::{fixture, rstest};
    use serde_json::json;
    use std::fs;

    #[fixture]
    #[once]
    fn shared_validator() -> PhenopacketSchemaValidator {
        PhenopacketSchemaValidator::default()
    }

    #[fixture]
    fn base_phenopacket() -> Value {
        let phenostr =
            fs::read_to_string(json_phenopacket_dir()).expect("Could not read test file");
        serde_json::from_str(&phenostr).expect("Invalid JSON in test file")
    }

    #[rstest]
    fn test_valid_base(shared_validator: &PhenopacketSchemaValidator, base_phenopacket: Value) {
        let res = shared_validator.validate_phenopacket(&base_phenopacket);
        assert!(
            res.is_ok(),
            "Base phenopacket should be valid: {:?}",
            res.err()
        );
    }

    #[rstest]
    fn test_missing_top_level_id(
        shared_validator: &PhenopacketSchemaValidator,
        mut base_phenopacket: Value,
    ) {
        base_phenopacket.as_object_mut().unwrap().remove("id");

        let res = shared_validator.validate_phenopacket(&base_phenopacket);

        assert!(res.is_err());
        let err_msg = res.unwrap_err().to_string();
        assert!(err_msg.contains("id"), "Error should mention missing 'id'");
    }

    #[rstest]
    fn test_invalid_deep_property(
        shared_validator: &PhenopacketSchemaValidator,
        mut base_phenopacket: Value,
    ) {
        if let Some(features) = base_phenopacket.pointer_mut("/phenotypicFeatures/0/type") {
            features.as_object_mut().unwrap().remove("id");
        } else {
            panic!("Test fixture does not have expected structure /phenotypicFeatures/0/type");
        }

        let res = shared_validator.validate_phenopacket(&base_phenopacket);

        assert!(
            res.is_err(),
            "Should fail when deep required field is missing"
        );
    }

    #[rstest]
    fn test_wrong_data_type(
        shared_validator: &PhenopacketSchemaValidator,
        mut base_phenopacket: Value,
    ) {
        if let Some(obj) = base_phenopacket.as_object_mut() {
            obj.insert(
                "phenotypicFeatures".to_string(),
                json!("Should be an array"),
            );
        }

        let res = shared_validator.validate_phenopacket(&base_phenopacket);

        assert!(res.is_err());
        let err = res.unwrap_err();
        assert!(err.to_string().to_lowercase().contains("array"));
    }

    #[rstest]
    fn test_invalid_enum_value(
        shared_validator: &PhenopacketSchemaValidator,
        mut base_phenopacket: Value,
    ) {
        if let Some(subject) = base_phenopacket.get_mut("subject") {
            subject
                .as_object_mut()
                .unwrap()
                .insert("sex".to_string(), json!("YES"));
        }

        let res = shared_validator.validate_phenopacket(&base_phenopacket);

        assert!(res.is_err());
        let err_msg = res.unwrap_err().to_string();
        assert!(err_msg.contains("YES") || err_msg.contains("is not"));
    }

    #[rstest]
    fn test_validator_thread_safety() {
        let validator = std::sync::Arc::new(PhenopacketSchemaValidator::default());
        let phenostr = fs::read_to_string(json_phenopacket_dir()).unwrap();
        let pp: Value = serde_json::from_str(&phenostr).unwrap();

        let mut handles = vec![];

        for _ in 0..5 {
            let v_clone = validator.clone();
            let pp_clone = pp.clone();
            handles.push(std::thread::spawn(move || {
                let res = v_clone.validate_phenopacket(&pp_clone);
                assert!(res.is_ok());
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
    }
}
