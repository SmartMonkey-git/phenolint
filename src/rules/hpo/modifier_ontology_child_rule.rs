use crate::diagnostics::specs::{DiagnosticSpec, LabelSpecs};
use crate::diagnostics::{LintFinding, LintReport, ReportSpecs};
use crate::enums::Patch;
use crate::error::RuleInitError;
use crate::json::{JsonCursor, Pointer};
use crate::rules::hpo::constants::{MODIFIER_ID, ONSET_ID, SEVERITY_ID};
use crate::rules::rule_registry::RuleRegistration;
use crate::traits::{LintRule, RuleCheck};
use crate::{FromContext, LinterContext, register_rule};
use codespan_reporting::diagnostic::{LabelStyle, Severity};
use ontolius::TermId;
use ontolius::ontology::HierarchyQueries;
use ontolius::ontology::csr::FullCsrOntology;
use phenolint_macros::lint_rule;
use serde_json::Value;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug)]
/// Validates that phenotypic feature modifiers are descendants of the Clinical Modifier term.
///
/// This rule implements the linting check `PF002`, which ensures that all modifiers
/// applied to phenotypic features are valid HPO clinical modifiers. According to the
/// HPO specification, modifiers must be descendants of "Clinical modifier" (HP:0012823).
///
/// # Rule Logic
///
/// For each phenotypic feature in the phenopacket:
/// 1. Iterates through all modifiers applied to the feature
/// 2. Checks if each modifier is a descendant of HP:0012823 (Clinical modifier)
/// 3. Reports a `NonModifier` violation if an invalid term is used as a modifier
///
/// # Example
///
/// Using "Generalized-onset seizure" (HP:0002197) as a modifier would be flagged as
/// invalid because it's a phenotypic abnormality term, not a clinical modifier.
/// Valid modifiers include terms like "Severe" (HP:0012828) or "Progressive" (HP:0003676),
/// which are descendants of HP:0012823.
#[lint_rule(id = "HPO001")]
pub struct ModifierOntologyChildRule {
    hpo: Arc<FullCsrOntology>,
    clinical_modifiers: TermId,
    onset: TermId,
}

impl ModifierOntologyChildRule {
    fn new(hpo: Arc<FullCsrOntology>) -> Self {
        ModifierOntologyChildRule {
            hpo,
            clinical_modifiers: TermId::from_str(MODIFIER_ID).unwrap(),
            onset: TermId::from_str(ONSET_ID).unwrap(),
        }
    }
}

impl FromContext for ModifierOntologyChildRule {
    fn from_context(context: &mut LinterContext) -> Result<Box<dyn RuleCheck>, RuleInitError> {
        match context.hpo() {
            None => Err(RuleInitError::NeedsHPO),
            Some(hpo) => Ok(Box::new(ModifierOntologyChildRule::new(hpo.clone()))),
        }
    }
}

impl RuleCheck for ModifierOntologyChildRule {
    fn check(&self, phenostr: &str, report: &mut LintReport) {
        let mut finding: Vec<LintFinding> = Vec::new();
        let mut cursor = JsonCursor::new(phenostr).expect("Should be valid JSON string");
        cursor.down("phenotypicFeatures");

        for feature_entries in cursor.peek() {
            if cursor
                .push_anchor()
                .down(feature_entries)
                .peek()
                .contains(&"modifiers".to_string())
            {
                for mod_idx in cursor.down("modifiers").peek() {
                    if let Some(Value::String(hpo_id)) =
                        cursor.down(format!("{}/id", mod_idx)).current_value()
                    {
                        match TermId::from_str(hpo_id) {
                            Ok(mod_term_id) => {
                                if !self
                                    .hpo
                                    .is_descendant_of(&mod_term_id, &self.clinical_modifiers)
                                {
                                    finding.push(LintFinding::new(
                                        Self::RULE_ID,
                                        self.write_report(&mut cursor),
                                        self.determine_patch(&mut cursor, mod_term_id),
                                    ))
                                }
                            }
                            Err(_) => continue,
                        }
                    }
                }
            }
            cursor.pop_anchor();
        }

        report.extend_finding(finding);
    }
}

impl ModifierOntologyChildRule {
    fn write_report(&self, cursor: &mut JsonCursor) -> ReportSpecs {
        cursor.push_anchor().up();

        let ont_class_span = cursor.span().expect("Should have found span");

        let mut labels = vec![LabelSpecs {
            style: LabelStyle::Primary,
            range: ont_class_span.0..ont_class_span.1,
            message: "Found non-modifier in modifier section".to_string(),
        }];

        if cursor.up().up().peek().contains(&"type".to_string()) {
            let type_span = cursor.down("type").span().expect("Should have found span");

            labels.push(LabelSpecs {
                style: LabelStyle::Secondary,
                range: type_span.0..type_span.1,
                message: "Found for this phenotypic feature.".to_string(),
            });
        } else {
            let pt_span = cursor.up().span().expect("Should have found span");
            labels.push(LabelSpecs {
                style: LabelStyle::Secondary,
                range: pt_span.0..pt_span.1,
                message: "In this section.".to_string(),
            });
        }

        let diagnostic_spec = DiagnosticSpec {
            severity: Severity::Warning,
            code: Some(Self::RULE_ID.to_string()),
            message: "Non-Modifier Found".to_string(),
            labels,
            notes: vec![format!(
                "Note: All Ontology class in modifiers need to be a child of '{}'",
                self.clinical_modifiers
            )],
        };
        cursor.pop_anchor();
        ReportSpecs::new(diagnostic_spec)
    }

    fn determine_patch(&self, cursor: &mut JsonCursor, id_in_modifiers: TermId) -> Option<Patch> {
        cursor.push_anchor().up();

        let target_pointer = cursor.pointer().clone();

        if self.hpo.is_descendant_of(&id_in_modifiers, &self.onset)
            && !cursor.up().up().down("onset").is_valid_position()
        {
            let onset_pointer = cursor.pointer().clone();
            cursor.pop_anchor();

            return Some(Patch::Move {
                from: target_pointer,
                to: onset_pointer,
            });
        }

        cursor.pop_anchor();
        Some(Patch::Remove { at: target_pointer })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::LintReport;
    use crate::test_utils::{HPO, LINT_CONTEXT, assert_report_message};
    use crate::traits::RuleCheck;
    use phenopackets::schema::v2::Phenopacket;
    use phenopackets::schema::v2::core::{
        OntologyClass, PhenotypicFeature, TimeElement, time_element,
    };
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    // --- Test Data Builders ---

    fn create_ontology_class(id: &str, label: &str) -> OntologyClass {
        OntologyClass {
            id: id.to_string(),
            label: label.to_string(),
        }
    }

    /// Helper to create a PhenotypicFeature with optional modifiers and onset
    fn create_phenotypic_feature(
        feature_type: OntologyClass,
        modifiers: Vec<OntologyClass>,
        onset: Option<OntologyClass>,
    ) -> PhenotypicFeature {
        PhenotypicFeature {
            r#type: Some(feature_type),
            modifiers,
            onset: onset.map(|o| TimeElement {
                element: Some(time_element::Element::OntologyClass(o)),
            }),
            ..Default::default()
        }
    }

    // --- Custom Assertion Helpers ---

    fn assert_patch_remove(patch: &Patch, exp_at: &str) {
        match patch {
            Patch::Remove { at } => {
                assert_eq!(
                    at.position(),
                    exp_at,
                    "Patch 'at' should point to the invalid modifier"
                );
            }
            _ => panic!("Expected Patch::Remove variant"),
        }
    }

    fn assert_patch_move(patch: &Patch, exp_from: &str, exp_to: &str) {
        match patch {
            Patch::Move { from, to } => {
                assert_eq!(
                    from.position(),
                    exp_from,
                    "Patch 'from' should point to the invalid modifier"
                );
                assert_eq!(
                    to.position(),
                    exp_to,
                    "Patch 'to' should point to the new onset location"
                );
            }
            _ => panic!("Expected Patch::Move variant"),
        }
    }

    // --- Test Runner ---

    /// Sets up the context and runs the rule check
    fn run_check(phenopacket: &Phenopacket) -> LintReport {
        // The rule is created from context, which is what we are testing
        let rule =
            ModifierOntologyChildRule::from_context(&mut LINT_CONTEXT.lock().unwrap()).unwrap();

        let mut report = LintReport::default();
        let json_str = serde_json::to_string_pretty(phenopacket).unwrap();

        rule.check(&json_str, &mut report);
        report
    }

    // --- Test Cases ---

    #[test]
    fn test_no_phenotypic_features() {
        let phenopacket = Phenopacket::default();
        let report = run_check(&phenopacket);
        assert!(report.violations().is_empty());
    }

    #[test]
    fn test_feature_no_modifiers() {
        let feature_term = create_ontology_class("HP:0000707", "Nervous system abnormality");
        let phenopacket = Phenopacket {
            phenotypic_features: vec![create_phenotypic_feature(feature_term, vec![], None)],
            ..Default::default()
        };
        let report = run_check(&phenopacket);
        assert!(report.violations().is_empty());
    }

    #[rstest]
    fn test_feature_valid_modifier() {
        // "Severe" (HP:0012828) is a child of "Clinical modifier" (HP:0012823)
        let feature_term = create_ontology_class("HP:0000707", "Nervous system abnormality");
        let valid_modifier = create_ontology_class("HP:0012828", "Severe");
        let phenopacket = Phenopacket {
            phenotypic_features: vec![create_phenotypic_feature(
                feature_term,
                vec![valid_modifier],
                None,
            )],
            ..Default::default()
        };
        let report = run_check(&phenopacket);
        assert!(report.violations().is_empty());
    }

    #[rstest]
    fn test_feature_multiple_valid_modifiers() {
        let feature_term = create_ontology_class("HP:0000707", "Nervous system abnormality");
        let valid_modifier_1 = create_ontology_class("HP:0012828", "Severe");
        let valid_modifier_2 = create_ontology_class("HP:0003676", "Progressive");
        let phenopacket = Phenopacket {
            phenotypic_features: vec![create_phenotypic_feature(
                feature_term,
                vec![valid_modifier_1, valid_modifier_2],
                None,
            )],
            ..Default::default()
        };
        let report = run_check(&phenopacket);
        assert!(report.violations().is_empty());
    }

    #[rstest]
    fn test_violation_invalid_modifier_remove() {
        // "Generalized-onset seizure" (HP:0002197) is NOT a modifier
        let feature_term = create_ontology_class("HP:0000707", "Nervous system abnormality");
        let invalid_modifier = create_ontology_class("HP:0002197", "Generalized-onset seizure");

        let phenopacket = Phenopacket {
            phenotypic_features: vec![create_phenotypic_feature(
                feature_term,
                vec![invalid_modifier],
                None,
            )],
            ..Default::default()
        };
        let report = run_check(&phenopacket.clone());

        assert_eq!(report.violations().len(), 1);
        let finding = report.findings().first().unwrap();

        assert_eq!(finding.violation().rule_id(), "HPO001");
        assert_report_message(
            &finding,
            ModifierOntologyChildRule::RULE_ID,
            "Non-Modifier Found",
            &serde_json::to_string_pretty(&phenopacket).unwrap(),
        );

        let patch = finding.patch().expect("Expected a patch");
        assert_patch_remove(patch, "/phenotypicFeatures/0/modifiers/0");
    }

    #[rstest]
    fn test_violation_invalid_modifier_move() {
        let feature_term = create_ontology_class("HP:0000707", "Nervous system abnormality");
        let onset_modifier = create_ontology_class("HP:0011463", "Childhood onset");

        let phenopacket = Phenopacket {
            phenotypic_features: vec![create_phenotypic_feature(
                feature_term,
                vec![onset_modifier],
                None, // <-- No onset exists
            )],
            ..Default::default()
        };
        let report = run_check(&phenopacket);

        assert_eq!(report.violations().len(), 1);
        let finding = report.findings().first().unwrap();

        assert_eq!(finding.violation().rule_id(), "HPO001");
        assert_report_message(
            finding,
            ModifierOntologyChildRule::RULE_ID,
            "Non-Modifier Found",
            "",
        );

        let patch = finding.patch().expect("Expected a patch");
        assert_patch_move(
            patch,
            "/phenotypicFeatures/0/modifiers/0",
            "/phenotypicFeatures/0/onset",
        );
    }

    #[rstest]
    fn test_violation_invalid_modifier_remove_onset_exists() {
        // "Childhood onset" (HP:0011463) is an Onset term
        let feature_term = create_ontology_class("HP:0000707", "Nervous system abnormality");
        let onset_modifier = create_ontology_class("HP:0011463", "Childhood onset");
        // "Juvenile onset" (HP:0003621) is the *existing* onset
        let existing_onset = create_ontology_class("HP:0003621", "Juvenile onset");

        let phenopacket = Phenopacket {
            phenotypic_features: vec![create_phenotypic_feature(
                feature_term,
                vec![onset_modifier],
                Some(existing_onset), // <-- Onset *already exists*
            )],
            ..Default::default()
        };
        let report = run_check(&phenopacket);

        assert_eq!(report.violations().len(), 1);
        let finding = report.findings().first().unwrap();

        assert_eq!(finding.violation().rule_id(), "HPO001");

        // The logic should fall back to Remove, not Move
        let patch = finding.patch().expect("Expected a patch");
        assert_patch_remove(patch, "/phenotypicFeatures/0/modifiers/0");
    }

    #[rstest]
    fn test_multiple_features_mixed_validity() {
        // Feature 0: Valid
        let feature_term_0 = create_ontology_class("HP:0000707", "Nervous system abnormality");
        let valid_modifier = create_ontology_class("HP:0012828", "Severe");
        let feature_0 = create_phenotypic_feature(feature_term_0, vec![valid_modifier], None);

        // Feature 1: Invalid (Remove)
        let feature_term_1 = create_ontology_class("HP:0001250", "Seizure");
        let invalid_modifier = create_ontology_class("HP:0002197", "Generalized-onset seizure");
        let feature_1 = create_phenotypic_feature(feature_term_1, vec![invalid_modifier], None);

        // Feature 2: Invalid (Move)
        let feature_term_2 = create_ontology_class("HP:0000708", "Behavioral abnormality");
        let onset_modifier = create_ontology_class("HP:0011463", "Childhood onset");
        let feature_2 = create_phenotypic_feature(feature_term_2, vec![onset_modifier], None);

        let phenopacket = Phenopacket {
            phenotypic_features: vec![feature_0, feature_1, feature_2],
            ..Default::default()
        };
        let report = run_check(&phenopacket);

        assert_eq!(report.violations().len(), 2);

        // Check finding 0 (from feature 1)
        let finding_0 = report.findings().get(0).unwrap();
        assert_eq!(finding_0.violation().rule_id(), "HPO001");
        assert_patch_remove(
            finding_0.patch().unwrap(),
            "/phenotypicFeatures/1/modifiers/0",
        );

        // Check finding 1 (from feature 2)
        let finding_1 = report.findings().get(1).unwrap();
        assert_eq!(finding_1.violation().rule_id(), "HPO001");
        assert_patch_move(
            finding_1.patch().unwrap(),
            "/phenotypicFeatures/2/modifiers/0",
            "/phenotypicFeatures/2/onset",
        );
    }

    #[rstest]
    fn test_multiple_invalid_modifiers_same_feature() {
        let feature_term = create_ontology_class("HP:0001250", "Seizure");
        // Invalid (Remove)
        let invalid_modifier_0 = create_ontology_class("HP:0002197", "Generalized-onset seizure");
        // Invalid (Move)
        let invalid_modifier_1 = create_ontology_class("HP:0011463", "Childhood onset");

        let phenopacket = Phenopacket {
            phenotypic_features: vec![create_phenotypic_feature(
                feature_term,
                vec![invalid_modifier_0, invalid_modifier_1],
                None,
            )],
            ..Default::default()
        };
        let report = run_check(&phenopacket);

        assert_eq!(report.violations().len(), 2);

        let finding_0 = report.findings().get(0).unwrap();
        assert_patch_remove(
            finding_0.patch().unwrap(),
            "/phenotypicFeatures/0/modifiers/0",
        );

        let finding_1 = report.findings().get(1).unwrap();
        assert_patch_move(
            finding_1.patch().unwrap(),
            "/phenotypicFeatures/0/modifiers/1",
            "/phenotypicFeatures/0/onset",
        );
    }

    #[rstest]
    fn test_skip_modifier_with_no_id() {
        // This test uses serde_json::json! to create a string that is
        // structurally valid JSON but would not be buildable from the
        // phenopacket structs directly. This tests the rule's JSON parsing logic.
        use serde_json::json;

        let rule =
            ModifierOntologyChildRule::from_context(&mut LINT_CONTEXT.lock().unwrap()).unwrap();
        let mut report = LintReport::default();

        let json_data = json!({
            "phenotypicFeatures": [
                {
                    "type": { "id": "HP:0000707", "label": "Nervous system abnormality" },
                    "modifiers": [
                        {
                            // "id" field is missing
                            "label": "Modifier with no id"
                        }
                    ]
                }
            ]
        });

        let json_str = serde_json::to_string_pretty(&json_data).unwrap();
        rule.check(&json_str, &mut report);

        // The rule should skip this modifier and not report a violation
        assert!(report.violations().is_empty());
    }
}
