use crate::diagnostics::LintViolation;
use crate::error::FromContextError;
use crate::rules::rule_registration::RuleRegistration;
use crate::rules::traits::{LintRule, RuleCheck, RuleFromContext, RuleMetaData};
use crate::tree::node_repository::List;
use crate::LinterContext;
use phenolint_macros::register_rule;
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
            .map(|r| r.materialized_node.namespace_prefix.as_str())
            .collect();

        let mut violations = vec![];

        let mut complained = HashSet::new();
        for node in data.0.iter() {
            if let Some(prefix) = find_prefix(node.materialized_node.id.as_str()) {
                if !known_prefixes.contains(prefix) && !complained.contains(prefix) {
                    violations.push(LintViolation::new(
                        LintRule::rule_id(self),
                        vec![node.pointer.clone()], // <- warns about the ontology class itself
                    ));
                    complained.insert(prefix);
                }
            }
        }
        violations
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
