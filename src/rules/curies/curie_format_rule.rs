use crate::diagnostics::{LintFinding, LintReport, OwnedReport};
use crate::error::RuleInitError;
use crate::json::{JsonCursor, Pointer};
use crate::linter_context::LinterContext;
use crate::register_rule;
use crate::rules::rule_registry::RuleRegistration;
use crate::traits::{FromContext, LintRule, RuleCheck};
use ariadne::{Label, Report, ReportKind};
use json_spanned_value::spanned::Value as SpannedValue;
use phenolint_macros::lint_rule;
use phenopackets::schema::v2::core::OntologyClass;
use regex::Regex;
use serde_json::Value;

#[derive(Debug, Default)]
#[lint_rule(id = "CURIE001")]
pub struct CurieFormatRule;

impl FromContext for CurieFormatRule {
    fn from_context(_: &LinterContext) -> Result<Box<dyn RuleCheck>, RuleInitError> {
        Ok(Box::new(CurieFormatRule))
    }
}

impl RuleCheck for CurieFormatRule {
    fn check(&self, phenostr: &str, report: &mut LintReport) {
        let regex = Regex::new("^[A-Z][A-Z0-9_]+:[A-Za-z0-9_]+$").unwrap();
        let cursor = JsonCursor::new(
            serde_json::from_str(phenostr)
                .unwrap_or_else(|_| panic!("Could not serialize phenopacket")),
        );

        for (pointer, value) in cursor.iter_with_paths() {
            if let Some(ont_class) = Self::get_ontology_class_from_value(value)
                && !regex.is_match(&ont_class.id)
            {
                report.push_finding(LintFinding::new(
                    Self::RULE_ID,
                    Self::write_report(phenostr, pointer.clone().down("id")),
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

    fn write_report(phenostr: &str, pointer: &Pointer) -> OwnedReport {
        let value: SpannedValue = json_spanned_value::from_str(phenostr)
            .unwrap_or_else(|_| panic!("Could not serialize phenopacket"));

        let (curie_start, curie_end) = value.pointer(pointer.position()).unwrap().span();
        let (context_span_start, context_span_end) = value
            .pointer(pointer.clone().up().position())
            .unwrap()
            .span();

        // ------

        let report_builder = Report::build(ReportKind::Error, ("stdin", curie_start..curie_end))
            .with_code(Self::RULE_ID)
            .with_message(format!("[{}] CURIE formatted incorrectly", Self::RULE_ID))
            .with_label(
                Label::new(("stdin", curie_start..curie_end))
                    .with_message("Expected CURIE with format CURIE:12345")
                    .with_priority(100),
            )
            .with_label(
                Label::new(("stdout", context_span_start..context_span_end))
                    .with_message("For this Ontology Class"),
            );

        let report = report_builder.finish();
        OwnedReport::new(report)
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
