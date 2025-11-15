use crate::LinterContext;
use crate::diagnostics::LintFinding;
use crate::error::RuleInitError;
use crate::parsing::traits::ParsableNode;
use crate::patches::patch_registry::PatchRegistry;
use crate::report::report_registry::ReportRegistry;
use crate::rules::rule_registry::LintingPolicy;
use crate::tree::node::Node;
use log::warn;
use phenopackets::schema::v2::core::{OntologyClass, PhenotypicFeature};

pub(crate) struct NodeRouter {
    enabled_rules: Vec<String>,
}

impl NodeRouter {
    pub fn new(enabled_rules: Vec<String>) -> Self {
        Self { enabled_rules }
    }
    pub fn lint_node(&self, node: &Node, context: &mut LinterContext) -> Vec<LintFinding> {
        if let Some(oc) = OntologyClass::parse(node) {
            self.route_to_rules(node, &oc, context)
        } else if let Some(pf) = PhenotypicFeature::parse(node) {
            self.route_to_rules(node, &pf, context)
        } else {
            vec![]
        }
    }

    fn route_to_rules<N>(
        &self,
        node: &Node,
        pared_node: &N,
        context: &mut LinterContext,
    ) -> Vec<LintFinding>
    where
        LintingPolicy<N>: inventory::Collect,
    {
        let mut findings = vec![];
        for rule in inventory::iter::<LintingPolicy<N>>() {
            if self.enabled_rules.iter().any(|s| s == rule.rule_id) {
                match (rule.factory)(context) {
                    Ok(rule_check) => {
                        let violations = rule_check.check(pared_node, node);

                        for violation in violations {
                            let patches = PatchRegistry::with_all_patches().get_patches_for(
                                rule.rule_id,
                                node,
                                &violation,
                            );

                            let report = ReportRegistry::with_all_reports().get_report_for(
                                rule.rule_id,
                                node,
                                &violation,
                            );

                            findings.push(LintFinding::new(violation, report, patches));
                        }
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
        findings
    }
}
