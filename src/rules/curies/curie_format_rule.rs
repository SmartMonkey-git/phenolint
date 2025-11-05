use annotate_snippets::Report;
use crate::rules::rule_registry::RuleRegistration;
use crate::linting_report::{LintReport, LintReportInfo, LintingViolation};
use crate::traits::{LintRule, RuleCheck};
use phenopackets::schema::v2::core::OntologyClass;
use regex::Regex;
use serde_json::Value;
use phenolint_macros::lint_rule;
use crate::register_rule;
use json_spanned_value::Value as SpannedValue;

#[derive(Debug, Default)]
#[lint_rule(id = "CURIE001")]
pub struct CurieFormatRule;
impl RuleCheck for CurieFormatRule {
    fn check(&self,phenobytes: &[u8], report: &mut LintReport) {
        let value = serde_json::from_slice(phenobytes)
            .unwrap_or_else(|_| panic!("Could not serialize phenopacket"));

        let mut stack = vec![&value];
        while let Some(current_value) = stack.pop() {
            if let Some(ont_class) = Self::get_ontology_class_from_value(&current_value) {
                let regex = Regex::new("^[A-Z][A-Z0-9_]+:[A-Za-z0-9_]+$").unwrap();
                if !regex.is_match(&ont_class.id) {
                    report.push_info(LintReportInfo::new(
                        LintingViolation::new(Self::RULE_ID, Self::write_report(phenobytes, &ont_class.id)),
                        None
                    ));
                }
            }

            match current_value {
                Value::Object(map) => {
                    for (_key, sub_value) in map {
                        stack.push(sub_value);
                    }
                }
                Value::Array(arr) => {
                    for sub_value in arr {
                        stack.push(sub_value);
                    }
                }
                _ => {}
            }
        }

    }


}
impl CurieFormatRule {


    fn get_ontology_class_from_value(value: &Value) -> Option<OntologyClass> {
        if let Value::Object(map) = &value
            && map.keys().len() == 2
            && map.contains_key("label")
            && map.contains_key("id")
            && let Ok(ont_class) = serde_json::from_value::<OntologyClass>(value.clone())
        {
            Some(ont_class)
        } else {
            None
        }
    }

    fn write_report<'a>(phenobytes: &[u8], wrong_curie: &str) -> Report<'static> {
        let value: SpannedValue = json_spanned_value::from_slice(phenobytes)
            .unwrap_or_else(|_| panic!("Could not serialize phenopacket"));


        Report::default()


    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use phenopackets::schema::v2::Phenopacket;
    use phenopackets::schema::v2::core::{Diagnosis, Interpretation, OntologyClass};
    use rstest::rstest;

    #[rstest]
    fn test_valid_curie_format() {
        let mut report = LintReport::new();
        let phenopacket = Phenopacket {
            id: "test-phenopacket".to_string(),
            interpretations: vec![Interpretation {
                diagnosis: Some(Diagnosis {
                    disease: Some(OntologyClass {
                        id: "MONDO:0000359".to_string(),
                        label: "spondylocostal dysostosis".to_string(),
                    }),
                    genomic_interpretations: vec![],
                }),
                ..Default::default()
            }],
            ..Default::default()
        };
        CurieFormatRule.check(&serde_json::to_string_pretty(&phenopacket).unwrap().as_bytes(), &mut report);
        assert!(report.violations().is_empty());
    }

    #[rstest]
    fn test_invalid_curie_format() {
        let mut report = LintReport::new();
        let phenopacket = Phenopacket {
            id: "test-phenopacket".to_string(),
            interpretations: vec![Interpretation {
                diagnosis: Some(Diagnosis {
                    disease: Some(OntologyClass {
                        id: "666".to_string(),
                        label: "spondylocostal dysostosis".to_string(),
                    }),
                    genomic_interpretations: vec![],
                }),
                ..Default::default()
            }],
            ..Default::default()
        };

        CurieFormatRule.check(&serde_json::to_string_pretty(&phenopacket).unwrap().as_bytes(), &mut report);
        assert!(!report.violations().is_empty());
    }


}
