use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::helper::non_empty_vec::NonEmptyVec;
use crate::report::compilers::disease_consistency_report::__LINKER_ERROR_MISSING_REPORT_STRUCT_FOR_INTER001;
use crate::rules::rule_registration::RuleRegistration;
use crate::rules::traits::RuleMetaData;
use crate::rules::traits::{LintRule, RuleCheck, RuleFromContext};
use crate::tree::node_repository::List;
use crate::tree::traits::Node;
use phenolint_macros::register_rule;
use phenopackets::schema::v2::core::{Diagnosis, Disease};

#[derive(Debug, Default)]
/// ### INTER001
/// ## What it does
/// Checks if all diseases found in the interpretation section are also present in the diseases section.
///
/// ## Why is this bad?
/// It is expected that the disease section contains all diseases associated with a patient.
#[register_rule(id = "INTER001")]
pub struct DiseaseConsistencyRule;

impl RuleFromContext for DiseaseConsistencyRule {
    fn from_context(_: &LinterContext) -> Result<Box<dyn LintRule>, FromContextError> {
        Ok(Box::new(Self))
    }
}

impl RuleCheck for DiseaseConsistencyRule {
    type Data<'a> = (List<'a, Diagnosis>, List<'a, Disease>);

    fn check(&self, data: Self::Data<'_>) -> Vec<LintViolation> {
        let mut violations = vec![];

        let disease_terms: Vec<(&str, &str)> = data
            .1
            .iter()
            .filter_map(|disease| {
                disease
                    .inner
                    .term
                    .as_ref()
                    .map(|oc| (oc.id.as_str(), oc.label.as_str()))
            })
            .collect();

        for diagnosis in data.0.iter() {
            if let Some(oc) = &diagnosis.inner.disease
                && !disease_terms.contains(&(oc.id.as_str(), oc.label.as_str()))
            {
                violations.push(LintViolation::new(
                    LintRule::rule_id(self),
                    NonEmptyVec::new(diagnosis.pointer().clone().down("disease").clone(), None),
                ))
            }
        }

        violations
    }
}
