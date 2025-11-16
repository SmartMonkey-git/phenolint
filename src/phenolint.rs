#![allow(unused)]

use crate::LinterContext;
use crate::diagnostics::LintReport;
use crate::diagnostics::report::PhenopacketData;
use crate::enums::InputTypes;
use crate::error::{InitError, LintResult, LinterError, ParsingError};
use crate::parsing::phenopacket_parser::PhenopacketParser;
use crate::patches::patch_registry::PatchRegistry;
use crate::report::parser::ReportParser;
use crate::report::report_registry::ReportRegistry;
use crate::router::NodeRouter;
use crate::traits::Lint;
use crate::tree::abstract_pheno_tree::AbstractPhenoTree;
use codespan_reporting::term::termcolor::Buffer;
use log::warn;
use phenopackets::schema::v2::Phenopacket;
use prost::Message;
use prost::bytes::{Buf, BufMut};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Phenolint {
    context: LinterContext,
    router: NodeRouter,
}

impl Phenolint {
    pub fn new(context: LinterContext, rule_ids: Vec<String>) -> Self {
        let report_registry = ReportRegistry::with_enabled_reports(rule_ids.as_slice(), &context);
        let patch_registry = PatchRegistry::with_enabled_patches(rule_ids.as_slice(), &context);
        Phenolint {
            context,
            router: NodeRouter::new(rule_ids, report_registry, patch_registry),
        }
    }

    fn emit(&mut self, phenostr: &str, report: &LintReport) {
        for (info, report_specs) in report
            .findings()
            .iter()
            .filter_map(|info| info.report().map(|rs| (info, rs)))
        {
            if ReportParser::emit(report_specs, phenostr).is_err() {
                warn!(
                    "Unable to parse and emit report for: '{}'",
                    info.violation().rule_id()
                );
            }
        }
    }
}

impl Lint<str> for Phenolint {
    fn lint(&mut self, phenostr: &str, patch: bool, quit: bool) -> LintResult {
        let mut report = LintReport::default();

        let apt: AbstractPhenoTree = match PhenopacketParser::to_tree(phenostr) {
            Ok(t) => t,
            Err(err) => return LintResult::err(LinterError::ParsingError(err)),
        };

        for node in apt.traverse() {
            let findings = self.router.lint_node(&node, &mut self.context);
            report.extend_finding(findings);
        }

        if !quit {
            self.emit(phenostr, &report);
        }

        // TODO: Apply patches here if patch=True
        let a: Value = serde_json::from_str(phenostr).unwrap();

        report.patched_phenopacket = Some(PhenopacketData::Text(a.to_string()));
        LintResult::ok(report)
    }
}

impl Lint<PathBuf> for Phenolint {
    fn lint(&mut self, phenopath: &PathBuf, patch: bool, quit: bool) -> LintResult {
        let phenodata = match fs::read(phenopath) {
            Ok(phenodata) => phenodata,
            Err(err) => {
                return LintResult::err(LinterError::InitError(InitError::IO(err)));
            }
        };

        self.lint(phenodata.as_slice(), patch, quit)
    }
}

impl Lint<[u8]> for Phenolint {
    fn lint(&mut self, phenodata: &[u8], patch: bool, quit: bool) -> LintResult {
        let (phenostr, input_type) = match PhenopacketParser::to_string(phenodata) {
            Ok(phenostr) => phenostr,
            Err(err) => {
                return LintResult::err(LinterError::ParsingError(err));
            }
        };
        let mut lint_result = self.lint(phenostr.as_str(), patch, quit);

        if let Some(patched_phenopacket) = lint_result.report.patched_phenopacket {
            lint_result.report.patched_phenopacket = Some(match patched_phenopacket {
                PhenopacketData::Text(phenotext) => match input_type {
                    InputTypes::Protobuf => {
                        let mut buf = Vec::new();

                        let phenopb: Phenopacket = serde_json::from_str(&phenotext)
                            .expect("Failed to deserialize patched phenopacket from JSON");

                        phenopb
                            .encode(&mut buf)
                            .expect("Failed to encode patched phenopacket to Protobuf");

                        PhenopacketData::Binary(buf)
                    }
                    _ => PhenopacketData::Binary(phenotext.as_bytes().to_vec()),
                },
                PhenopacketData::Binary(phenobytes) => PhenopacketData::Binary(phenobytes),
            })
        };

        lint_result
    }
}
