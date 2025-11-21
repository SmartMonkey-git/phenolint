use crate::LinterContext;
use crate::diagnostics::LintFinding;
use crate::parsing::traits::ParsableNode;

use crate::rules::rule_registration::RuleRegistration;
use crate::rules::rule_registry::RuleRegistry;
use crate::tree::node::Node;
use crate::tree::pointer::Pointer;
use log::{error, warn};
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature, VitalStatus};

pub(crate) struct NodeSupplier;

impl NodeSupplier {
    pub fn supply_rules(&mut self, node: &Node, rule_registry: &mut RuleRegistry) {
        if let Some(oc) = OntologyClass::parse(node) {
            self.supply_rule(&oc, &node.pointer, rule_registry)
        } else if let Some(pf) = PhenotypicFeature::parse(node) {
            self.supply_rule(&pf, &node.pointer, rule_registry)
        } else if let Some(pp) = Phenopacket::parse(node) {
            self.supply_rule(&pp, &node.pointer, rule_registry)
        } else if let Some(vt) = VitalStatus::parse(node) {
            self.supply_rule(&vt, &node.pointer, rule_registry)
        } else {
            error!(
                "Unable to parse node at '{}'. Phenopacket schema might be invalid.",
                node.pointer
            );
        }
    }

    fn supply_rule<N: 'static>(
        &mut self,
        parsed_node: &N,
        pointer: &Pointer,
        rule_registry: &mut RuleRegistry,
    ) {
        for rule in inventory::iter::<RuleRegistration>() {
            if let Some(rule) = rule_registry.get_mut(rule.rule_id) {
                rule.supply_node_any(parsed_node, &pointer)
            }
        }
    }
    /*
    fn route_to_rules<N>(
        &self,
        node: &Node,
        parsed_node: &N,
        context: &mut LinterContext,
    ) -> Vec<LintFinding>
    where
        RuleRegistration: inventory::Collect,
    {
        let mut findings = vec![];
        for rule_res in inventory::iter::<RuleRegistration>() {
            if self.enabled_rules.iter().any(|s| s == rule_res.rule_id) {
                match (rule_res.factory)(context).as_ref() {
                    Ok(rule) => {
                        let violations = rule.check();

                        for violation in violations {
                            let patches = self.patch_registry.get_patches_for(
                                rule_res.rule_id,
                                node,
                                &violation,
                            );

                            let report = self.report_registry.get_report_for(
                                rule_res.rule_id,
                                node,
                                &violation,
                            );

                            findings.push(LintFinding::new(violation, report, patches));
                        }
                    }
                    Err(err) => warn!("From Context Error: {}", err),
                }
            }
        }
        findings
    }*/
}
