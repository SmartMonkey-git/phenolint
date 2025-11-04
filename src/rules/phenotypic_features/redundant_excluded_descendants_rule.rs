use crate::linting_report::{LintReport, LintReportInfo, LintingViolation};
use crate::rules::utils;
use crate::traits::{ RuleCheck};
use ontolius::ontology::csr::FullCsrOntology;
use phenopackets::schema::v2::Phenopacket;
use std::sync::Arc;


/// Validates that excluded phenotypic terms don't have redundant excluded descendants.
///
/// This rule implements the linting check `PF008`, which identifies cases where both an
/// ancestor term and its descendant terms are marked as excluded in a phenopacket. When
/// a general phenotype is excluded, all of its more specific descendants are implicitly
/// excluded as well, making their explicit exclusion redundant.
///
/// # Rule Logic
///
/// For each excluded phenotypic term in the phenopacket:
/// 1. Determines if the term is a "progenitor" (has no excluded ancestors itself)
/// 2. Finds all descendant terms that are also explicitly excluded
/// 3. Reports a `ExcludedDescendents` violation if redundant descendants are found
///
/// # Example
///
/// If "Abnormal heart morphology" (HP:0001627) is excluded, then excluding its
/// descendant "Ventricular septal defect" (HP:0001629) would be flagged as redundant,
/// since the more general exclusion already covers all specific heart defects.
#[derive(Debug)]
//#[lint_rule(id = "PF008")]
struct RedundantExcludedDescendantsRule {
    hpo: Arc<FullCsrOntology>,
}

impl RedundantExcludedDescendantsRule {
    pub fn new(hpo: Arc<FullCsrOntology>) -> Self {
        Self { hpo }
    }
}

impl RuleCheck for RedundantExcludedDescendantsRule {
    fn check(&self, phenopacket: &Phenopacket, report: &mut LintReport) {
        let (_, excluded) = utils::partition_phenotypic_features(phenopacket);

        // Case 3: Invalidate all descendents of a family for an excluded term
        // Because, if you can exclude a general phenotype the specific one can also be excluded.
        excluded.iter().for_each(|phenotypic_term| {
            let is_progenitor =
                utils::find_descendents(self.hpo.clone(), &excluded, phenotypic_term).is_empty();

            if is_progenitor {
                let child_terms =
                    utils::find_descendents(self.hpo.clone(), &excluded, phenotypic_term);
                if !child_terms.is_empty() {
                    // TODO: Add empty check
                    report.push_info(LintReportInfo::new(
                        LintingViolation::new("PF008", ""),
                        None,
                    ))
                }
            }
        });
    }


}


//TODO: Tests missing