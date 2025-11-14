use crate::LinterContext;
use crate::diagnostics::{LintFinding, LintReport, ReportParser, ReportSpecs};
use crate::enums::Patch;
use crate::error::{LintResult, LinterError};
use crate::new::abstract_pheno_tree::AbstractPhenoTree;
use crate::new::phenopacket_parser::PhenopacketParser;
use crate::new::router::NodeRouter;
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

        let apt: AbstractPhenoTree = match PhenopacketParser::to_tree(phenobytes) {
            Ok(t) => t,
            Err(err) => return LintResult::err(LinterError::InitError(err)),
        };

        for node in apt.traverse() {
            let findings = NodeRouter::lint_node(&node, &mut self.context);
            report.extend_finding(findings);
        }

        // TODO: Maybe this should be part of the CLI. If not, then we should convert the reports to Strings here and return them with the report. The CLI will just emit the Strings.
        if !quite {
            match PhenopacketParser::to_string(phenobytes) {
                Ok(phenostr) => {
                    for info in report.findings() {
                        if let Some(report_specs) = info.report() {
                            if let Err(err) = ReportParser::emit(report_specs, &phenostr) {
                                warn!(
                                    "Unable to parse and emit report for: '{}'",
                                    info.violation().rule_id()
                                );
                            };
                        }
                    }
                }
                Err(err) => {
                    warn!("Unable to parse phenopacket data into String: '{}'", err);
                }
            }
        }

        // TODO: Apply patches here if patch=True

        LintResult::ok(report)
    }
}
