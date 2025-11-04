use crate::linting_report::{LintReport, LintReportInfo, LintingViolation};
use crate::rules::utils;
use crate::traits::{ RuleCheck};
use ontolius::ontology::csr::FullCsrOntology;
use phenopackets::schema::v2::Phenopacket;
use std::sync::Arc;
use annotate_snippets::Report;

/// Validates that observed phenotypic terms don't have redundant observed ancestors.
///
/// This rule implements the linting check `PF007`, which identifies cases where both a
/// specific phenotypic term and its more general ancestor terms are marked as observed
/// in a phenopacket. Best practice in phenotype annotation is to use the most specific
/// terms possible. When a specific phenotype is observed, annotating its general
/// ancestors adds no additional information and should be avoided.
///
/// # Rule Logic
///
/// For each observed phenotypic term in the phenopacket:
/// 1. Determines if the term is a "scion" (has no observed descendants, making it most specific)
/// 2. Finds all ancestor terms that are also explicitly observed
/// 3. Reports an `ObservedAncestor` violation if redundant ancestors are found
///
/// # Example
///
/// If "Ventricular septal defect" (HP:0001629) is marked as observed, then also marking
/// its ancestor "Abnormal heart morphology" (HP:0001627) as observed would be flagged
/// as redundant. The specific term already conveys all the information of the general
/// term, making the ancestor annotation unnecessary.
//#[lint_rule(id = "PF007")]
struct ObservedAncestorRule {
    hpo: Arc<FullCsrOntology>,
}

impl ObservedAncestorRule {
    fn new(hpo: Arc<FullCsrOntology>) -> Self {
        ObservedAncestorRule { hpo }
    }
}

impl RuleCheck for ObservedAncestorRule {
    fn check(&self, phenopacket: &Phenopacket, report: &mut LintReport) {
        let (observed, _) = utils::partition_phenotypic_features(phenopacket);

        // Invalidate all ancestors of a family for an observed term
        // Amongst the observed terms, we want to keep the most specific ones.
        // Which means, if we find a term that is more general then another, we deem the more general term invalid.
        observed.iter().for_each(|phenotypic_term| {
            let is_scion =
                utils::find_descendents(self.hpo.clone(), &observed, phenotypic_term).is_empty();

            if is_scion {
                let ancestor_terms =
                    utils::find_ancestors(self.hpo.clone(), &observed, phenotypic_term);

                if !ancestor_terms.is_empty() {
                    // TODO: Add empty check
                    report.push_info(LintReportInfo::new(
                        LintingViolation::new("PF007", Report::default()), None
                    ))
                }
            }
        });
    }


}

#[cfg(test)]
mod tests {
    use annotate_snippets::Report;
    use crate::test_utils::HPO;

    use crate::linting_report::{LintReport, LintingViolation};
    use crate::rules::phenotypic_features::observed_ancestor_rule::ObservedAncestorRule;
    use crate::traits::RuleCheck;
    use phenopackets::schema::v2::Phenopacket;
    use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature};
    use rstest::rstest;
    use pretty_assertions::assert_eq;

    #[rstest]
    fn test_find_related_phenotypic_features_case_1() {
        let rule = ObservedAncestorRule::new(HPO.clone());
        let expected_progenitor = OntologyClass {
            id: "HP:0000448".to_string(),
            label: "Prominent nose".to_string(),
        };
        let expected_middle = OntologyClass {
            id: "HP:0005105".to_string(),
            label: "Abnormal nasal morphology".to_string(),
        };
        let expected_ancestor = OntologyClass {
            id: "HP:0000366".to_string(),
            label: "Abnormality of the nose".to_string(),
        };
        let phenopacket = Phenopacket {
            phenotypic_features: vec![
                PhenotypicFeature {
                    r#type: Some(expected_middle.clone()),
                    ..Default::default()
                },
                PhenotypicFeature {
                    r#type: Some(expected_ancestor.clone()),
                    ..Default::default()
                },
                PhenotypicFeature {
                    r#type: Some(expected_progenitor.clone()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        let mut report = LintReport::new();
        rule.check(&phenopacket, &mut report);

        let violations = report.violations();
        assert_eq!(violations.first().unwrap().clone(), LintingViolation::new("PF007", Report::default()));
    }
}
