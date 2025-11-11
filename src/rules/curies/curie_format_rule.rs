use crate::diagnostics::specs::{DiagnosticSpec, LabelSpecs};
use crate::diagnostics::{LintFinding, LintReport, ReportSpecs};
use crate::error::RuleInitError;
use crate::json::JsonCursor;
use crate::linter_context::LinterContext;
use crate::register_rule;
use crate::rules::rule_registry::RuleRegistration;
use crate::traits::{FromContext, LintRule, RuleCheck};
use codespan_reporting::diagnostic::{LabelStyle, Severity};
use phenolint_macros::lint_rule;
use phenopackets::schema::v2::core::OntologyClass;
use regex::Regex;
use serde_json::Value;

#[derive(Debug, Default)]
#[lint_rule(id = "CURIE001")]
pub struct CurieFormatRule;

impl FromContext for CurieFormatRule {
    fn from_context(_: &mut LinterContext) -> Result<Box<dyn RuleCheck>, RuleInitError> {
        Ok(Box::new(CurieFormatRule))
    }
}

impl RuleCheck for CurieFormatRule {
    fn check(&self, phenostr: &str, report: &mut LintReport) {
        let regex = Regex::new("^[A-Z][A-Z0-9_]+:[A-Za-z0-9_]+$").unwrap();
        let cursor = JsonCursor::new(phenostr).expect("Phenopacket is not a valid json");

        for (pointer, value) in cursor.iter_with_paths() {
            if let Some(ont_class) = Self::get_ontology_class_from_value(value)
                && !regex.is_match(&ont_class.id)
            {
                let mut temp_cursor =
                    JsonCursor::new(phenostr).expect("Phenopacket is not a valid json");
                report.push_finding(LintFinding::new(
                    Self::RULE_ID,
                    //TODO: no clone here
                    Self::write_report(temp_cursor.point_to(&pointer)),
                    None,
                ));
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

    fn write_report(cursor: &mut JsonCursor) -> ReportSpecs {
        cursor.push_anchor();
        let (curie_start, curie_end) = cursor.down("id").span().expect("Should have found span");
        cursor.up();

        let (context_span_start, context_span_end) =
            cursor.up().span().expect("Should have found span");

        cursor.up().up();
        if let Some(val) = cursor.current_value()
            && val.as_object().is_some()
        {
            cursor.up();
        };
        let (label_start, label_end) = cursor.span().expect("Should have found span");

        let labels = vec![
            LabelSpecs {
                style: LabelStyle::Primary,
                range: curie_start..curie_end,
                message: "Expected CURIE with format CURIE:12345".to_string(),
            },
            LabelSpecs {
                style: LabelStyle::Secondary,
                range: context_span_start..context_span_end,
                message: "For this Ontology Class".to_string(),
            },
            LabelSpecs {
                style: LabelStyle::Secondary,
                range: label_start..label_end,
                message: "In this section".to_string(),
            },
        ];

        let diagnostic_spec = DiagnosticSpec {
            severity: Severity::Error,
            code: Some(Self::RULE_ID.to_string()),
            message: "CURIE formatted incorrectly".to_string(),
            labels,
            notes: vec![
                "Note: All CURIE IDs need to follow the format: ^[A-Z][A-Z0-9_]+:[A-Za-z0-9_]+$"
                    .to_string(),
            ],
        };

        ReportSpecs::new(diagnostic_spec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::assert_report_message;
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
        CurieFormatRule.check(
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
            &mut report,
        );
        assert!(report.violations().is_empty());
    }

    #[rstest]
    fn test_invalid_curie_format() {
        let wrong_curie = "not_a_curie".to_string();
        let mut report = LintReport::new();
        let phenopacket = Phenopacket {
            id: "test-phenopacket".to_string(),
            interpretations: vec![Interpretation {
                diagnosis: Some(Diagnosis {
                    disease: Some(OntologyClass {
                        id: wrong_curie.clone(),
                        label: "spondylocostal dysostosis".to_string(),
                    }),
                    genomic_interpretations: vec![],
                }),
                ..Default::default()
            }],
            ..Default::default()
        };

        CurieFormatRule.check(
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
            &mut report,
        );
        assert!(!report.violations().is_empty());
        let findings = report.findings().first().unwrap();

        assert_report_message(
            findings,
            CurieFormatRule::RULE_ID,
            "Expected CURIE",
            &serde_json::to_string_pretty(&phenopacket).unwrap(),
        );
    }
}
