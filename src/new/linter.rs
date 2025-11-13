use crate::LinterContext;
use crate::diagnostics::{LintFinding, LintReport};
use crate::enums::Patch;
use crate::error::{LintResult, LinterError, RuleInitError};
use crate::new::phenopacket_tree::AbstractPhenoTree;
use crate::new::router::NodeRouter;
use crate::new::tree_factory::TreeFactory;
use crate::rules::rule_registry::LintingPolicy;
use log::warn;
use std::vec;

pub struct Linter {
    context: LinterContext,
}

impl Linter {
    pub fn new(context: LinterContext) -> Self {
        Linter { context }
    }
    pub fn lint(&mut self, phenobytes: &[u8], patch: bool, quite: bool) -> LintResult {
        let mut report = LintReport::default();

        let apt: AbstractPhenoTree = match TreeFactory::try_parse(phenobytes) {
            Ok(t) => t,
            Err(err) => return LintResult::err(LinterError::InitError(err)),
        };

        for node in apt.traverse() {
            let violations = NodeRouter::lint_node(&node, &mut self.context);

            for violation in violations {
                // TODO: Compile Patches here from the violations
                let finding = LintFinding::new(violation, vec![Patch::default()]);
                report.push_finding(finding);
            }
        }

        /*
        if !quite {
            for info in report.findings() {
                if let Err(err) = ReportParser::emit(info.violation().report(), phenostr) {
                    warn!(
                        "Unable to parse and emit report for: '{}'",
                        info.violation().rule_id()
                    );
                };
            }
        }*/

        // TODO: Apply patches here if patch=True

        LintResult::ok(report)
    }
}
