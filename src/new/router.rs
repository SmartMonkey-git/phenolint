use crate::LinterContext;
use crate::diagnostics::LintViolation;
use crate::error::RuleInitError;
use crate::new::node::Node;
use crate::new::traits::ParsableNode;
use crate::rules::rule_registry::LintingPolicy;
use log::warn;
use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature};

pub struct NodeRouter;

impl NodeRouter {
    pub fn lint_node(node: &Node, context: &mut LinterContext) -> Vec<LintViolation> {
        if let Some(oc) = OntologyClass::parse(node) {
            Self::route_to_rules(&oc, context)
        } else if let Some(pf) = PhenotypicFeature::parse(node) {
            Self::route_to_rules(&pf, context)
        } else {
            vec![]
        }
    }

    fn route_to_rules<N>(pared_node: &N, context: &mut LinterContext) -> Vec<LintViolation>
    where
        LintingPolicy<N>: inventory::Collect,
    {
        let mut violations = vec![];
        for rule in inventory::iter::<LintingPolicy<N>>() {
            if context.rule_ids().iter().any(|s| s == rule.rule_id) {
                println!("------");
                match (rule.factory)(context) {
                    Ok(rule) => {
                        let finding = rule.check(pared_node);
                        violations.extend(finding);
                    }

                    Err(err) => match err {
                        RuleInitError::NeedsHPO => {
                            warn!(
                                "Rule '{}'  was configured, but needs the HPO. HPO not found or not configured.",
                                rule.rule_id,
                            );
                        }
                    },
                }
            }
        }
        violations
    }
}
