use crate::LinterContext;
use crate::diagnostics::LintFinding;
use crate::error::FromContextError;
use crate::parsing::traits::ParsableNode;
use crate::patches::patch_registry::PatchRegistry;
use crate::report::report_registry::ReportRegistry;
use crate::rules::rule_registry::LintingPolicy;
use crate::rules::traits::LintRule;
use crate::tree::node::Node;
use crate::tree::pointer::Pointer;
use log::{error, warn};
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature, VitalStatus};

pub(crate) struct NodeRouter {
    enabled_rules: Vec<String>,
    report_registry: ReportRegistry,
    patch_registry: PatchRegistry,
}

impl NodeRouter {
    pub fn new(
        enabled_rules: Vec<String>,
        report_registry: ReportRegistry,
        patch_registry: PatchRegistry,
    ) -> Self {
        Self {
            enabled_rules,
            report_registry,
            patch_registry,
        }
    }
    pub fn lint_node(&self, node: &Node, context: &mut LinterContext) -> Vec<LintFinding> {
        if let Some(oc) = OntologyClass::parse(node) {
            self.route_to_rules(node, &oc, context)
        } else if let Some(pf) = PhenotypicFeature::parse(node) {
            self.route_to_rules(node, &pf, context)
        } else if let Some(pf) = Phenopacket::parse(node) {
            self.route_to_rules(node, &pf, context)
        } else if let Some(vt) = VitalStatus::parse(node) {
            self.route_to_rules(node, &vt, context)
        } else {
            error!(
                "Unable to parse node at '{}'. Phenopacket schema might be invalid.",
                node.pointer
            );
            vec![]
        }
    }

    fn supply_rules<N>(&self, parsed_node: &N, pointer: Pointer, context: &mut LinterContext) {
        for rule in inventory::iter::<LintingPolicy>() {
            match (rule.factory)(context).lock() {
                Ok(rule) => rule.supply_node_any(parsed_node, &pointer),
                Err(err) => warn!("From Context Error: {}", err),
            }
        }
    }

    fn route_to_rules<N>(
        &self,
        node: &Node,
        parsed_node: &N,
        context: &mut LinterContext,
    ) -> Vec<LintFinding>
    where
        LintingPolicy: inventory::Collect,
    {
        let mut findings = vec![];
        for rule_res in inventory::iter::<LintingPolicy>() {
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
    }
}
