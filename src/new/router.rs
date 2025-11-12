use crate::diagnostics::LintReport;
use crate::error::RuleInitError;
use crate::new::json_traverser::JsonNode;
use crate::rules::rule_registry::LintingPolicy;
use crate::{LinterContext, Node, NodeParser};
use log::warn;
use phenopackets::schema::v2::core::OntologyClass;
use serde_json::Value;
use std::marker::PhantomData;

pub struct NodeRouter<T, P: NodeParser<T>> {
    _marker_t: PhantomData<T>,
    _marker_p: PhantomData<P>,
}

impl<T, P: NodeParser<T>> NodeRouter<T, P> {
    pub fn route_value(value: &Box<dyn Node<T>>, context: &LinterContext, report: &mut LintReport)
    // Add other parsable structs here
    where
        P: NodeParser<T>,
    {
        if let Some(oc) = P::parse_ontology_class(value) {
            NodeRouter::<T, OntologyClass>::route_to_rules(&oc, context, report);
            println!("Parsed OntologyClass: {:?}", oc);
        } else {
            println!("Failed to parse as OntologyClass");
        }
    }

    fn route_to_rules(parsed_value: &P, context: &LinterContext, report: &mut LintReport)
    where
        T: 'static,
        P: NodeParser<Value>,
    {
        for rule in inventory::iter::<LintingPolicy<T>>.into_iter() {
            if context.rule_ids().iter().any(|s| s == rule.rule_id) {
                match (rule.factory)(&context) {
                    Ok(rule) => {
                        rule.check(&parsed_value, report);
                    }
                    Err(err) => match err {
                        RuleInitError::NeedsHPO => {
                            warn!(
                                "Rule '{}' needs the HPO. HPO not found or not configured",
                                rule.rule_id
                            );
                        }
                    },
                }
            }
        }
    }
}
