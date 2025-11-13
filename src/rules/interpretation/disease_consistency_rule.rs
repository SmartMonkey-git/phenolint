use crate::LinterContext;
use crate::diagnostics::specs::{DiagnosticSpec, LabelSpecs};
use crate::diagnostics::{LintFinding, LintReport, LintViolation, ReportSpecs};

use crate::error::RuleInitError;
use crate::json::{JsonCursor, Pointer};
use crate::register_rule;
use crate::rules::rule_registry::{BoxedRuleCheck, LintingPolicy};
use crate::traits::{FromContext, LintRule, RuleCheck};
use codespan_reporting::diagnostic::{LabelStyle, Severity};
use phenolint_macros::lint_rule;
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::OntologyClass;

#[derive(Debug, Default)]
/// Validates that diseases in interpretations are also present in the diseases list.
///
/// This rule implements the linting check `INTER001`, which ensures consistency between
/// the diseases mentioned in interpretations and those listed in the top-level diseases
/// field of a phenopacket. When a disease is diagnosed in an interpretation, it should
/// also appear in the phenopacket's diseases list for proper data consistency and
/// completeness.
///
/// # Rule Logic
///
/// 1. Extracts all disease terms from interpretation diagnoses
/// 2. Extracts all disease terms from the top-level diseases field
/// 3. Identifies diseases that appear in interpretations but not in the diseases list
/// 4. Reports a `DiseaseConsistency` violation for each missing disease
/// 5. Suggests a `Duplicate` fix action to copy the disease to the diseases list
///
/// # Example
///
/// If an interpretation contains a diagnosis of "Marfan syndrome" (OMIM:154700) but
/// this disease does not appear in the phenopacket's diseases field, the rule will
/// flag this inconsistency. The disease should be added to both locations to maintain
/// data integrity across the phenopacket structure.
#[lint_rule(id = "INTER001")]
pub struct DiseaseConsistencyRule;

impl FromContext for DiseaseConsistencyRule {
    type CheckType = Phenopacket;

    fn from_context(_: &LinterContext) -> Result<BoxedRuleCheck<Phenopacket>, RuleInitError> {
        Ok(Box::new(Self))
    }
}

impl RuleCheck for DiseaseConsistencyRule {
    type CheckType = Phenopacket;

    fn check(&self, phenostr: &Phenopacket) -> Vec<LintViolation> {
        todo!()
    }
}

impl DiseaseConsistencyRule {
    fn write_report(cursor: &mut JsonCursor) -> ReportSpecs {
        cursor.push_anchor();
        let (inter_disease_start, inter_disease_end) = cursor.span().expect("Should have a span");

        let mut primary_message = "Diseases found in interpretations".to_string();
        let secondary_message = "that was not present in diseases section";

        let mut labels = Vec::new();

        if cursor
            .root()
            .point_to(&Pointer::new("/diseases"))
            .current_value()
            .is_some()
        {
            let (start, end) = cursor.span().expect("Should have a span");
            labels.push(LabelSpecs {
                style: LabelStyle::Secondary,
                range: start..end,
                message: secondary_message.to_string(),
            });
        } else {
            primary_message = format!("{primary_message} {secondary_message}");
        }

        labels.push(LabelSpecs {
            style: LabelStyle::Primary,
            range: inter_disease_start..inter_disease_end,
            message: primary_message,
        });

        let diagnostic_spec = DiagnosticSpec {
            severity: Severity::Warning,
            code: Some(Self::RULE_ID.to_string()),
            message: "Disease Inconsistency".to_string(),
            labels,
            notes: Vec::new(),
        };
        cursor.pop_anchor();
        ReportSpecs::new(diagnostic_spec)
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_utils::assert_report_message;
    use phenopackets::schema::v2::Phenopacket;
    use phenopackets::schema::v2::core::{Diagnosis, Disease, Interpretation, OntologyClass};
    use rstest::rstest;

    fn assert_patch(patch: &Patch, exp_from: &str, exp_to: &str) {
        match patch {
            Patch::Duplicate { from, to } => {
                assert_eq!(
                    from.position(),
                    exp_from,
                    "Patch 'from' should point to interpretation disease"
                );

                assert_eq!(
                    to.position(),
                    exp_to,
                    "Patch 'to' should point to diseases/1/term"
                );
            }
            _ => panic!("Expected Patch::Duplicate variant"),
        };
    }

    fn create_ontology_class(id: &str, label: &str) -> OntologyClass {
        OntologyClass {
            id: id.to_string(),
            label: label.to_string(),
        }
    }

    fn create_disease(term: OntologyClass) -> Disease {
        Disease {
            term: Some(term),
            ..Default::default()
        }
    }

    fn create_interpretation(disease: Option<OntologyClass>) -> Interpretation {
        Interpretation {
            diagnosis: disease.map(|d| Diagnosis {
                disease: Some(d),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_validate_no_interpretations_no_diseases() {
        let phenopacket = Phenopacket::default();
        let mut report = LintReport::default();

        DiseaseConsistencyRule.check(
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
            &mut report,
        );

        assert!(report.violations().is_empty());
    }

    #[test]
    fn test_validate_interpretation_matches_disease() {
        let disease_term = create_ontology_class("MONDO:0007254", "Breast Cancer");
        let disease = create_disease(disease_term.clone());
        let interpretation = create_interpretation(Some(disease_term));

        let phenopacket = Phenopacket {
            diseases: vec![disease],
            interpretations: vec![interpretation],
            ..Default::default()
        };
        let mut report = LintReport::default();

        DiseaseConsistencyRule.check(
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
            &mut report,
        );

        assert!(report.violations().is_empty());
    }

    #[rstest]
    fn test_validate_interpretation_disease_mismatch() {
        let disease_term = create_ontology_class("MONDO:0007254", "Breast Cancer");
        let interpretation_term = create_ontology_class("MONDO:0005148", "Diabetes");

        let disease = create_disease(disease_term);
        let interpretation = create_interpretation(Some(interpretation_term.clone()));

        let phenopacket = Phenopacket {
            diseases: vec![disease],
            interpretations: vec![interpretation],
            ..Default::default()
        };
        let mut report = LintReport::default();

        DiseaseConsistencyRule.check(
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
            &mut report,
        );

        let violations = report.violations();
        assert_eq!(violations.len(), 1);
        let finding = report.findings().first().unwrap();

        assert_eq!(finding.violation().rule_id(), "INTER001");
        let patch = finding.patch().expect("Expected a patch to be present");

        assert_patch(
            patch,
            "/interpretations/0/diagnosis/disease",
            "/diseases/1/term",
        );
        assert_report_message(
            finding,
            DiseaseConsistencyRule::RULE_ID,
            "Disease Inconsistency",
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
        )
    }

    #[test]
    fn test_validate_multiple_interpretations_all_match() {
        let disease1 = create_ontology_class("MONDO:0007254", "Breast Cancer");
        let disease2 = create_ontology_class("MONDO:0005148", "Diabetes");

        let diseases = vec![
            create_disease(disease1.clone()),
            create_disease(disease2.clone()),
        ];
        let interpretations = vec![
            create_interpretation(Some(disease1)),
            create_interpretation(Some(disease2)),
        ];

        let phenopacket = Phenopacket {
            diseases,
            interpretations,
            ..Default::default()
        };
        let mut report = LintReport::default();

        DiseaseConsistencyRule.check(
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
            &mut report,
        );

        assert!(report.violations().is_empty());
    }

    #[test]
    fn test_validate_multiple_interpretations_some_mismatch() {
        let disease1 = create_ontology_class("MONDO:0007254", "Breast Cancer");
        let disease2 = create_ontology_class("MONDO:0005148", "Diabetes");
        let disease3 = create_ontology_class("MONDO:0005015", "Hypertension");

        let diseases = vec![
            create_disease(disease1.clone()),
            create_disease(disease2.clone()),
        ];
        let interpretations = vec![
            create_interpretation(Some(disease1)),
            create_interpretation(Some(disease3.clone())),
        ];

        let phenopacket = Phenopacket {
            diseases,
            interpretations,
            ..Default::default()
        };
        let mut report = LintReport::default();

        DiseaseConsistencyRule.check(
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
            &mut report,
        );

        let finding = report.findings().first().unwrap();

        let violations = report.violations();
        assert_eq!(violations.len(), 1);

        let patch = finding.patch().expect("Expected a patch to be present");

        assert_patch(
            patch,
            "/interpretations/1/diagnosis/disease",
            "/diseases/2/term",
        );

        assert_report_message(
            finding,
            DiseaseConsistencyRule::RULE_ID,
            "Disease Inconsistency",
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
        )
    }

    #[test]
    fn test_validate_interpretation_without_diagnosis() {
        let disease_term = create_ontology_class("MONDO:0007254", "Breast Cancer");
        let disease = create_disease(disease_term);

        let interpretation = Interpretation {
            diagnosis: None,
            ..Default::default()
        };

        let phenopacket = Phenopacket {
            diseases: vec![disease],
            interpretations: vec![interpretation],
            ..Default::default()
        };
        let mut report = LintReport::default();

        DiseaseConsistencyRule.check(
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
            &mut report,
        );

        assert!(report.violations().is_empty());
    }

    #[test]
    fn test_validate_interpretation_with_diagnosis_but_no_disease() {
        let disease_term = create_ontology_class("MONDO:0007254", "Breast Cancer");
        let disease = create_disease(disease_term);

        let interpretation = Interpretation {
            diagnosis: Some(Diagnosis {
                disease: None,
                ..Default::default()
            }),
            ..Default::default()
        };

        let phenopacket = Phenopacket {
            diseases: vec![disease],
            interpretations: vec![interpretation],
            ..Default::default()
        };
        let mut report = LintReport::default();

        DiseaseConsistencyRule.check(
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
            &mut report,
        );

        assert!(report.violations().is_empty());
    }

    #[test]
    fn test_validate_disease_without_term() {
        let interpretation_term = create_ontology_class("MONDO:0007254", "Breast Cancer");

        let disease = Disease {
            term: None,
            ..Default::default()
        };
        let interpretation = create_interpretation(Some(interpretation_term.clone()));

        let phenopacket = Phenopacket {
            diseases: vec![disease],
            interpretations: vec![interpretation],
            ..Default::default()
        };
        let mut report = LintReport::default();

        DiseaseConsistencyRule.check(
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
            &mut report,
        );
        let violations = report.violations();

        let finding = report.findings().first().unwrap();

        assert_eq!(violations.len(), 1);
        let patch = finding.patch().expect("Expected a patch to be present");

        assert_patch(
            patch,
            "/interpretations/0/diagnosis/disease",
            "/diseases/0/term",
        );
        assert_report_message(
            finding,
            DiseaseConsistencyRule::RULE_ID,
            "Disease Inconsistency",
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
        )
    }

    #[test]
    fn test_validate_multiple_mismatches() {
        let disease1 = create_ontology_class("MONDO:0007254", "Breast Cancer");
        let interpretation1 = create_ontology_class("MONDO:0005148", "Diabetes");
        let interpretation2 = create_ontology_class("MONDO:0005015", "Hypertension");

        let diseases = vec![create_disease(disease1)];
        let interpretations = vec![
            create_interpretation(Some(interpretation1.clone())),
            create_interpretation(Some(interpretation2.clone())),
        ];

        let phenopacket = Phenopacket {
            diseases,
            interpretations,
            ..Default::default()
        };
        let mut report = LintReport::default();

        DiseaseConsistencyRule.check(
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
            &mut report,
        );

        let finding = report.findings().first().unwrap();

        assert_eq!(report.findings().len(), 2);
        let patch = finding.patch().expect("Expected a patch to be present");

        assert_patch(
            patch,
            "/interpretations/0/diagnosis/disease",
            "/diseases/1/term",
        );
        assert_report_message(
            finding,
            DiseaseConsistencyRule::RULE_ID,
            "Disease Inconsistency",
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
        )
    }

    #[test]
    fn test_validate_duplicate_interpretation_diseases() {
        let disease_term = create_ontology_class("MONDO:0007254", "Breast Cancer");
        let disease = create_disease(disease_term.clone());

        let interpretations = vec![
            create_interpretation(Some(disease_term.clone())),
            create_interpretation(Some(disease_term)),
        ];

        let phenopacket = Phenopacket {
            diseases: vec![disease],
            interpretations,
            ..Default::default()
        };
        let mut report = LintReport::default();

        DiseaseConsistencyRule.check(
            serde_json::to_string_pretty(&phenopacket).unwrap().as_str(),
            &mut report,
        );

        assert!(report.violations().is_empty());
    }
}
*/
