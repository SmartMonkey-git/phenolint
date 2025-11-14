#![allow(unused)]

use crate::LinterContext;
use crate::diagnostics::LintReport;
use crate::error::{LintResult, LinterError};
use crate::parsing::phenopacket_parser::PhenopacketParser;
use crate::report::parser::ReportParser;
use crate::router::NodeRouter;
use crate::tree::abstract_pheno_tree::AbstractPhenoTree;
use log::warn;

pub struct Linter {
    context: LinterContext,
}

impl Linter {
    pub fn new(context: LinterContext) -> Self {
        Linter { context }
    }
    pub fn lint(&mut self, phenobytes: &[u8], patch: bool, quit: bool) -> LintResult {
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
        if !quit {
            self.emit(phenobytes, &report);
        }

        // TODO: Apply patches here if patch=True

        LintResult::ok(report)
    }

    fn emit(&mut self, phenobytes: &[u8], report: &LintReport) {
        let phenostr = match PhenopacketParser::to_string(phenobytes) {
            Ok(s) => s,
            Err(err) => {
                warn!("Unable to parse phenopacket data into String: '{}'", err);
                return;
            }
        };

        for (info, report_specs) in report
            .findings()
            .iter()
            .filter_map(|info| info.report().map(|rs| (info, rs)))
        {
            if ReportParser::emit(report_specs, &phenostr).is_err() {
                warn!(
                    "Unable to parse and emit report for: '{}'",
                    info.violation().rule_id()
                );
            }
        }
    }
}
