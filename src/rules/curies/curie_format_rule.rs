use crate::diagnostics::{LintFinding, LintReport, OwnedReport};
use crate::json::{JsonCursor, Pointer};
use crate::linter_context::LinterContext;
use crate::register_rule;
use crate::rules::rule_registry::RuleRegistration;
use crate::traits::{FromContext, LintRule, RuleCheck};
use annotate_snippets::{AnnotationKind, Level, Snippet};
use json_spanned_value::spanned::Value as SpannedValue;
use phenolint_macros::lint_rule;
use phenopackets::schema::v2::core::OntologyClass;
use regex::Regex;
use serde_json::Value;

#[derive(Debug, Default)]
#[lint_rule(id = "CURIE001")]
pub struct CurieFormatRule;

impl FromContext for CurieFormatRule {
    fn from_context(_: &LinterContext) -> Option<Box<dyn RuleCheck>> {
        Some(Box::new(CurieFormatRule))
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

        let report = Level::WARNING
            .primary_title(format!("[{}] CURIE formatted incorrectly", Self::RULE_ID))
            .element(
                Snippet::source(phenostr.to_string())
                    .annotation(
                        AnnotationKind::Primary
                            .span(curie_start..curie_end)
                            .label("Expected CURIE with format CURIE:12345"),
                    )
                    .annotation(
                        AnnotationKind::Context
                            .span(context_span_start..context_span_end)
                            .label("For this Ontology Class"),
                    ),
            );

        OwnedReport::new(report)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::parser::ReportParser;
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
        let report_info = report.findings.first().unwrap();

        ReportParser::emit(report_info.violation().report());
        let parsed_report = ReportParser::parse(report_info.violation().report());
        assert!(parsed_report.contains(&wrong_curie))
    }
}
