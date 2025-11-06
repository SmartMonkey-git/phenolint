use crate::linter_context::LinterContext;
use crate::linting_report::{LintReport, LintReportInfo, LintingViolation};
use crate::register_rule;
use crate::rules::rule_registry::RuleRegistration;
use crate::rules::utils::json_cursor::{JsonCursor, Pointer};
use crate::traits::{FromContext, LintRule, RuleCheck};
use annotate_snippets::renderer::DecorStyle;
use annotate_snippets::{AnnotationKind, Level, Renderer, Report, Snippet};
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
    fn check(&self, phenobytes: &[u8], report: &mut LintReport) {
        let cursor = JsonCursor::new(
            serde_json::from_slice(phenobytes)
                .unwrap_or_else(|_| panic!("Could not serialize phenopacket")),
        );

        for (pointer, value) in cursor.iter_with_paths() {
            if let Some(ont_class) = Self::get_ontology_class_from_value(&value) {
                let regex = Regex::new("^[A-Z][A-Z0-9_]+:[A-Za-z0-9_]+$").unwrap();
                if !regex.is_match(&ont_class.id) {
                    report.push_info(LintReportInfo::new(
                        LintingViolation::new(
                            Self::RULE_ID,
                            Self::write_report(phenobytes, pointer.clone().down("id")),
                        ),
                        None,
                    ));
                }
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

    fn write_report<'a>(phenobytes: &[u8], pointer: &Pointer) -> Report<'static> {
        let json = String::from_utf8(phenobytes.to_vec()).unwrap();
        let value: SpannedValue = json_spanned_value::from_str(&json)
            .unwrap_or_else(|_| panic!("Could not serialize phenopacket"));
        let (curie_start, curie_end) = value.pointer(pointer.position()).unwrap().span();
        let (context_span_start, context_span_end) = value
            .pointer(pointer.clone().up().position())
            .unwrap()
            .span();

        let report = &[Level::WARNING
            .primary_title(format!("[{}] CURIE formatted incorrectly", Self::RULE_ID))
            .element(
                Snippet::source(json)
                    //.line_start(190 - 2)
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
            )];

        let renderer = Renderer::styled().decor_style(DecorStyle::Unicode);
        anstream::println!("{}", renderer.render(report));
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
        CurieFormatRule.check(
            &serde_json::to_string_pretty(&phenopacket)
                .unwrap()
                .as_bytes(),
            &mut report,
        );
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
                        id: "not_a_curie".to_string(),
                        label: "spondylocostal dysostosis".to_string(),
                    }),
                    genomic_interpretations: vec![],
                }),
                ..Default::default()
            }],
            ..Default::default()
        };

        CurieFormatRule.check(
            &serde_json::to_string_pretty(&phenopacket)
                .unwrap()
                .as_bytes(),
            &mut report,
        );
        assert!(!report.violations().is_empty());
    }

    #[rstest]
    fn test_iter() {
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
    }
}
