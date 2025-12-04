use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::report::enums::{LabelPriority, ViolationSeverity};
use crate::report::report_registration::ReportRegistration;
use crate::report::specs::{LabelSpecs, ReportSpecs};
use crate::report::traits::RuleReport;
use crate::report::traits::{CompileReport, RegisterableReport, ReportFromContext};
use crate::rules::rule_registration::RuleRegistration;
use crate::rules::traits::{LintRule, RuleCheck, RuleFromContext, RuleMetaData};
use crate::tree::node_repository::List;
use crate::tree::pointer::Pointer;
use crate::tree::traits::Node;
use phenolint_macros::{register_report, register_rule};
use phenopackets::schema::v2::core::{OntologyClass, Resource};
use std::collections::HashSet;

/// ### INTER002
/// ## What it does
/// Check that a phenopacket contains a resource for each used CURIEs.
///
/// ## Why is this bad?
/// Phenopacket Schema prescribes that all ontology concepts need a `Resource`
/// to document the ontology's version, or to allow CURIE 👉 IRI expansion.
#[register_rule(id = "INTER002")]
struct CuriesHaveResourcesRule;

impl RuleFromContext for CuriesHaveResourcesRule {
    fn from_context(_context: &LinterContext) -> Result<Box<dyn LintRule>, FromContextError>
    where
        Self: Sized,
    {
        Ok(Box::new(Self))
    }
}

impl RuleCheck for CuriesHaveResourcesRule {
    type Data<'a> = (List<'a, OntologyClass>, List<'a, Resource>);

    fn check(&self, data: Self::Data<'_>) -> Vec<LintViolation> {
        let known_prefixes: HashSet<_> = data
            .1
            .iter()
            .map(|r| r.inner.namespace_prefix.as_str())
            .collect();

        let mut violations = vec![];

        for node in data.0.iter() {
            if let Some(prefix) = find_prefix(node.inner.id.as_str())
                && !known_prefixes.contains(prefix)
            {
                violations.push(LintViolation::new(
                    ViolationSeverity::Error,
                    LintRule::rule_id(self),
                    node.pointer().clone().into(), // <- warns about the ontology class itself
                ));
            }
        }
        violations
    }
}

#[cfg(test)]
mod test_curies_have_resources {
    use crate::rules::resources::CuriesHaveResourcesRule;
    use crate::rules::traits::{RuleCheck, RuleMetaData};
    use crate::tree::node::MaterializedNode;
    use crate::tree::node_repository::List;
    use crate::tree::pointer::Pointer;
    use phenopackets::schema::v2::core::OntologyClass;

    #[test]
    fn check_that_a_term_needs_a_resource() {
        let rule = CuriesHaveResourcesRule;

        let ocs = [MaterializedNode::new(
            OntologyClass {
                id: "HP:0001250".into(),
                label: "Seizure".into(),
            },
            Default::default(),
            Pointer::new("/phenotypicFeatures/0/type"),
        )];
        let resources = [];
        let data = (List(&ocs), List(&resources));

        let violations = rule.check(data);

        assert_eq!(violations.len(), 1);
        let violation = violations.first().unwrap();

        assert_eq!(violation.rule_id(), rule.rule_id());
        assert_eq!(
            violation.at().first().unwrap().position(),
            "/phenotypicFeatures/0/type"
        );
    }
}

#[register_report(id = "INTER002")]
pub struct CuriesHaveResourcesReport;

impl ReportFromContext for CuriesHaveResourcesReport {
    fn from_context(_: &LinterContext) -> Result<Box<dyn RegisterableReport>, FromContextError> {
        Ok(Box::new(Self))
    }
}

impl CompileReport for CuriesHaveResourcesReport {
    fn compile_report(&self, full_node: &dyn Node, lint_violation: &LintViolation) -> ReportSpecs {
        let resources_ptr = Pointer::new("/metaData/resources");
        let span = if let Some(resources_range) = full_node.span_at(&resources_ptr).cloned() {
            resources_range
        } else {
            // `metaData` lacks the `resources` field itself.
            let metadata_ptr = Pointer::new("/metaData");
            full_node.span_at(&metadata_ptr)
                .cloned()
                .expect("We assume `metaData` is always in the `Node` because we validate the basic phenopacket invariants before running this rule")
        };

        ReportSpecs::from_violation(
            lint_violation,
            "An ontology class needs a resource".to_string(),
            vec![
                LabelSpecs::new(
                    LabelPriority::Primary,
                    full_node
                        .span_at(lint_violation.first_at())
                        .cloned()
                        .expect("Should be there"),
                    "This ontology class ...".to_string(),
                ),
                LabelSpecs::new(
                    LabelPriority::Secondary,
                    span,
                    "... should have a resource here".to_string(),
                ),
            ],
            vec![
                "Phenopacket Schema prescribes that all ontology classes need a resource to document the version of the used ontology, or to support CURIE -> IRI expansion.".to_string(),
            ]
        )
    }
}

fn find_prefix(curie: &str) -> Option<&str> {
    if let Some(idx) = curie.find(":") {
        Some(&curie[..idx])
    } else if let Some(idx) = curie.find("_") {
        Some(&curie[..idx])
    } else {
        None
    }
}

#[cfg(test)]
mod test_find_prefix {
    use super::find_prefix;

    #[test]
    fn test_find_prefix() {
        assert_eq!("HP", find_prefix("HP:0001250").unwrap());
        assert_eq!("HP", find_prefix("HP_0001250").unwrap());
        assert!(find_prefix("HP-0001250").is_none());
    }
}
